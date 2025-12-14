# wpgen CLI 概览
<!-- 角色：使用配置者 | 最近验证：2025-12-11 -->

用途：生成样本数据，配合 getting_started 用例与本地回归。

运行语义（重要更新）
- count：总产出条数。启动时按 `parallel` 精确按 worker 均分，余数前置到前若干个 worker；各 worker 各自跑完“本地任务量”即退出，总量严格等于 `count`。
- speed：全局速率（records/sec）。
  - `speed = 0` 表示无限制（不等待）。
  - `speed > 0` 时每 worker 速率为 `floor(speed / parallel)`；单位批量自适应，低速更平滑。
- parallel：生成 worker 的并行数。
  - 对 `blackhole_sink`，消费端也会随之并行（多消费者）；对其它 sink 默认单消费者（避免克隆外部连接）。
- 退出行为：生成组先完成；随后向 router/sinks/monitor 广播 Stop；同时这些组件在“通道关闭”时也会自然退出（双保险）。

诊断日志（关键标记）
- 生成 worker：`gen worker start …` / `gen worker(start sample|rule): limit : …` / `gen data task: … end` / `gen worker end`
- Router：`router start` / `router recv stop …` / `router: main channel closed` / `router exit`
- Sink：`sink recv cmd …` / `sink dat channel closed; exit`
- Monitor：`monitor proc start …` / `monitor recv cmd …` / `monitor channel closed; exit` / `monitor proc end`

常用流程
- `wpgen conf init` 初始化配置
- `wpgen sample -n <N> --stat <sec> [--out-path <FILE>]` 生成 N 条样本并周期输出统计

提示
- 生成文件通常位于 `./data/in_dat/`；可在 `wpgen` 配置里调整目标路径。
- `--out-path <FILE>`：覆盖配置中的输出路径（仅文件型 sink 生效）。

- `output.connect` 解析规则：从 `models/sinks` 起向上最多 32 层查找 `connectors/sink.d`。错误信息会打印起点与绝对路径，便于排查（常见误写：`syslog_udp_src` 应为 `syslog_udp_sink`）。

常见问题
- “不能退出”：检查是否出现 Router/Sink/Monitor 的 `… channel closed; exit` 或 `router recv stop … / router exit` 日志；确保运行的是 release 版本（日志才完整）。
- “产出不足”：`count` 将被精确分配给每个 worker；若日志里 `limit : …` 小于预期，多半是旧二进制或本地任务量在代码里被重复除以 `parallel`（现已修复）。

示例
```bash
# 覆盖输出路径并避免覆盖
wpgen sample -n 1000 --out-path ./data/in_dat/gen.dat

# 规则生成
wpgen rule -n 500 --out-path ./data/out_dat/rule.json
```
