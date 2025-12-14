# Prometheus syslog case 调试记录

## 现象
- `wpgen sample` 经 `syslog_tcp_sink` 写向 `127.0.0.1:1514`，运行中偶尔报 `Connection reset by peer / Broken pipe`。
- `wproj data validate` 提示 sink miss 超出比率；`data/logs/miss.dat` 记录 strip 后的 CSV 原文。
- `wparse.log` 多次出现 `monitor rec miss`，且在 TCP 源日志中固定只看到 `conn … produced 33 events`。

## 初步分析
- TCP dump 证明三次握手正确，`wpgen` 在几十毫秒内连续推送大量 payload；`wparse` 端正常 ACK 后在 ~0.1s 主动发 `FIN`，随后 `wpgen` 的写入收到 `RST`。
- `wproj rule parse` 验证 sample 与 WPL 相符，说明 strip/解析逻辑基本正确。
- 33 条/批与 `src/sources/tcp/conn/connection.rs` 中 `MAX_BATCH_BYTES = 30 * 1024` 一致：每批最多 30KB → 单条 ~900B 时恰好 33 条。并不是“读 33 条就停”，而是安全批处理上限。
- 真正导致连接断开的原因仍是 wparse 侧在读完当前 batch 后没有继续 `receive`，而是回到 `awaiting connectors`（可能脚本 stop，或 `wpgen` 客户端在服务端读完第一批后立即断开）。

## 调试增强
- syslog sink (`src/sinks/backends/syslog.rs`) 新增 send 序号与字节数日志，便于确认 `wpgen` 实际写到了哪个 seq。
- TCP 源 (`src/sources/tcp/source.rs`) 日志增强：begin try/blocking read 时记录 pending 队列；每批产出时输出 `batch_bytes`、`pending_after`，并在 close/error 分支打印具体 peer 与原因。
- syslog strip 钩子 (`src/sources/syslog/tcp_source.rs`) 加入 fast strip fallback / strip empty span 的 DEBUG 日志及 payload 预览。
- 调整 `MAX_BATCH_BYTES` → 40KB，减少单批固定 33 条的情况。

## 当前结论
- Broken pipe 源于 wparse 在处理完一批后立即关闭连接；并非 strip 崩溃或 syslog sink 发送过快。
- 需要在运行脚本时保持 wparse 不被提前 stop，或让 `wpgen` 在连接仍然打开时持续发送下一批。可按日志对照 send seq 与 `conn … produced …`，定位哪批之后连接被关闭。

## 后续建议
1. 跑 `core/prometheus_metrics/run.sh` 前清空旧日志，保持 `ctrl=debug,data=debug`，使用新的日志字段定位每次连接的关闭点。
2. 如果 `wparse.log` 显示 `conn … closed during …`，说明客户端先断开；若只剩 `awaiting connectors`，需要查看脚本是否在 `wpgen sample` 后立即 stop 了 wparse。
3. 如需进一步验证，可暂时将 `header_mode = "keep"`，确认 miss sink 记录 strip 前原文，排除 strip 失败因素。

## 重构计划：TcpSource 持有连接生命周期
为了避免 `syslog_1-picker` 的阻塞读取超时时立即 drop 连接，需要改写 `TcpSource` 的连接管理方式。拟采用以下方案：

1. **引入 ConnectionGuard**：在 `receive()` 中不再直接 `remove` 连接并在 async 函数中拥有其所有权，而是创建一个 `ConnectionGuard<'a>`，持有 `&'a mut TcpSource` 与 `conn_id`。guard 负责：
   - 构造时从 `HashMap` 拿出 `TcpConnection`；
   - `read()` 完成 try/blocking 逻辑，并返回“重新排队”或“注销”两种结果；
   - 在 `Drop` 中兜底：若 guard 在 `await` 期间被取消且尚未显式 requeue/deregister，就自动把连接放回 `connection_order`。这样即使 `tokio::select!` 超时，连接也不会凭空消失。

2. **显式回收/注销**：`read()` 根据 `ReadOutcome` 调用 `self.push_connection_back` 或 `self.deregister_connection`。只有在真正遇到 peer close/错误时才移除连接；正常路径始终重新排队，保证同一连接可以被下次 `receive()` 继续使用。

3. **错误处理**：`ConnectionGuard::read` 返回 `SourceResult<Option<SourceBatch>>`；若发生错误，guard 会先注销连接，再把 `SourceError` 返给上层，以便 picker 能正确统计失败。这样所有连接状态变化都集中在 guard 内部，不再依赖 `read_from_connection` 返回枚举，由 `TcpSource` 自己掌控生命周期。

4. **日志不变**：沿用现有的 try/blocking 日志（`produced`, `closed during ...` 等），便于排查；额外的 `warn_ctrl` 仅在 guard 构造失败（不应出现）或 Drop 兜底回队时打印一次，帮助发现异常路径。

依据该方案，可以彻底解决“阻塞 fetch 超时导致 wparse 主动关闭连接”的根因，后续实现时只需在 `src/sources/tcp/source.rs` 中引入 guard 并更新 `receive()`/`read_from_connection` 即可。
