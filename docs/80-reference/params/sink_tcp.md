# Sink:tcp 参数参考

通用 TCP 输出，支持 `line`（行分帧）与 `len`（长度前缀）两种模式。

参数
- `addr`（string，默认 `127.0.0.1`）：目标地址
- `port`（int，默认 `9000`）：目标端口
- `framing`（string，默认 `line`）：`line|len`
- `max_backoff`（bool，可选）：启用“发送队列感知 backoff”。仅在“无限速”场景才有意义，有限速时会被忽略。

连接器示例
```toml
[[connectors]]
id = "tcp_sink"
type = "tcp"
allow_override = ["addr", "port", "framing", "max_backoff"]
[connectors.params]
addr = "127.0.0.1"
port = 19000
framing = "line"

## 发送队列感知 Backoff（仅在无限速时自动生效）

- 痛点：无限速发送会堆积到内核发送队列，造成尾部丢失或对端压力过大。
- 方案：探测内核发送队列占用（Linux: SIOCOUTQ；macOS/BSD: SO_NWRITE），在占用过高时短暂 `sleep` 退让。
- 启停：
  - `wparse`：当 `[performance].rate_limit_rps = 0`，引擎自动开启 backoff；否则关闭。
  - `wpgen`：当 `[generator].speed = 0`，若未显式写入 `max_backoff`，自动开启；否则按配置。
- 配置：
  - 仅一个开关 `max_backoff`（布尔）；不暴露其它细粒度阈值/时长，避免过度复杂。
  - 默认采用自适应 backoff：目标占用 30%（带回差），按采样周期动态调整 `sleep_ms`，拥塞时逐步增大退让、拥塞解除后降为 0ms。
- 实现细节（摘录）：
  - 采样间隔：每 64 次写入探测一次；固定常量，后续如需调整再统一变更。
  - drain：退出时按 10ms 间隔轮询队列是否已清空，尽量避免尾部丢失。

```
