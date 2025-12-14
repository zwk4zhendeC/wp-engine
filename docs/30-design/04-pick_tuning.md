# 批/队列/拉取策略调优（内存/吞吐折中）

本文记录采集→解析链路的“批/队列/拉取”关键参数及四种推荐档位：当前、内存优先、吞吐优先、均衡。用于性能压测与生产参数选型参考。

> 术语与位置：
> - 源端批：`src/sources/tcp/conn/connection.rs`
> - 采集 pending/拉取水位：`src/runtime/collector/realtime/constants.rs`
> - 解析通道容量：`crates/wp-config/src/limits.rs`
> - TCP 每连接接收缓冲：`wpsrc.toml` 的 `params.tcp_recv_bytes`
> - shrink（空闲收缩）：`src/sources/tcp/conn/connection.rs`、`src/sources/net/tcp/connection.rs`

| 参数/含义 | 当前 | 内存优先 | 吞吐优先 | 均衡 |
|---|---|---|---|---|
| DEFAULT_BATCH_CAPACITY（事件/批） | 256 | 256 | 512 | 256–384 |
| MAX_BATCH_BYTES（字节/批上限） | 128KB | 128KB（或 96KB） | 256KB | 128–192KB |
| 小批合并触发（pending≥N） | 32 | 16–24 | 48–64 | 24–32 |
| 小批合并上限（合并后事件数） | 128 | 128（或 96） | 256 | 128–192 |
| PARSER_CHANNEL_CAP_DEFAULT（解析通道批数） | 512 | 256 | 1024 | 512–768 |
| PICKER_PENDING_CAPACITY（采集待投队列） | 64 | 32–64 | 128 | 64–96 |
| PullPolicy 水位（LO/HI ×burst） | 2 / 3 | 1 / 2 | 2 / 4 | 2 / 3 |
| tcp_recv_bytes（每连接用户态接收缓冲） | 256KB | 256KB（或 128KB） | 512KB–1MB | 256–512KB |
| TCP 缓冲 shrink（高水位→目标） | 1MB → 256KB | 1MB → 256KB | 1MB → 256KB | 1MB → 128–256KB |
| Net TCP 缓冲 shrink（高水位→目标） | 1MB → 256KB | 1MB → 256KB | 1MB → 256KB | 1MB → 128–256KB |
| parse_workers（解析并发） | ≈CPU | ≤CPU（防抖动） | 2×CPU（有核可用时） | ≈CPU |
| --speed-limit（采集限速） | 视运行时 | 设为“可持续 RPS”（pending<16） | 0（不限速） | 适度限速 |

## 选型建议
- 内存优先：限制通道/队列上限、早停拉、适度限速，搭配较大的单批，内存稳定但峰值吞吐低于极限。
- 吞吐优先：放宽通道/队列、批更大/更早合并，CPU 打满时内存峰值较高、回落较慢。
- 均衡（采用）：保留较大的单批（事件/字节），通道 512–768，水位 2/3，tcp_recv_bytes 256–512KB，shrink 1MB→128–256KB，parse_workers≈CPU，必要时少量限速。

## 调整入口与注意事项
- 源端批：`DEFAULT_BATCH_CAPACITY`、`MAX_BATCH_BYTES`（源读取帧后构批）
- 合并小批：`PICKER_COALESCE_TRIGGER`、`PICKER_COALESCE_MAX_EVENTS`（pending 高水位时合并）
- 解析通道：`PARSER_CHANNEL_CAP_DEFAULT`（按“批数”计数，批更大更不易满）
- 采集 pending：`PICKER_PENDING_CAPACITY`（过大推高 RSS，优先靠“批更大/通道更大/并发更多”）
- 拉取水位：`PICKER_PULL_LO/H I_MULTIPLIER`（更早停拉降低 pending 峰值时长）
- 每连接缓冲：`params.tcp_recv_bytes`（过大易抬高基线；不足会临时扩容，不影响功能）
- shrink：空闲时收缩过大缓冲（普通/零拷贝路径已启用；阈值/目标见上）
- 压测建议：CPU 打满即极限；再提升通道/队列只会抬内存，建议优先多连接/多实例/多进程（REUSEPORT）横向扩展。

> 变更记录：2025-12 采用“均衡”参数作为默认基线；如需切换档位，仅按表调整对应常量/配置即可。

