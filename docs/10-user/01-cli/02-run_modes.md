# 运行模式与退出策略
<!-- 角色：使用配置者 | 最近验证：2025-12-11 -->

本文档面向使用者，说明 wparse 在不同运行模式下（batch/daemon）的退出判定、行为差异、以及与命令行参数的关系。若需背景与设计细节，可参考 `../../30-decision/01-architecture.md`。

## TL;DR
- 默认运行模式是 daemon（常驻服务）。
- 批处理请显式指定：`wparse batch`。
- 批处理（batch）下不启动 acceptor；当所有文件源的采集线程（picker）结束后，主组完成，进程优雅退出。
  - 注意：batch 模式下会忽略网络类源（kind=`tcp` 以及 `syslog` 的 `protocol=tcp`）；如需接收 TCP 数据请使用 daemon 模式。
- 关键日志：
  - 每个源结束时：`数据源 '...' picker 正常结束`
  - 全局收尾：`all routine group await end!`

## CLI 用法
- 指定运行模式（默认 daemon）：
  - `wparse daemon` 适合服务化、长连接/网络接入等场景。
  - `wparse batch` 适合离线/文件源，读完后即整体退出。

### 子命令（推荐）
- 为了更直观，wparse/wprescue 原生支持以下子命令（无需别名转换）：
  - `wparse daemon [args]`（兼容 `deamon` 拼写）等价于 `wparse work --run-mode=daemon [args]`
  - `wparse batch  [args]` 等价于 `wparse work -b [args]`
  - 示例：`wparse batch -n 3000 --stat 2 --print_stat`
- 常用组合：
  - 指定最大处理条数：`-n <line_max>`（示例：`wparse batch -n 3000`）。
  - 打印统计：`--stat <sec> --print_stat`。

## 退出判定（批处理）
- 单源（picker）结束条件（任一满足）：
  - 上游 EOF → 采集线程优雅结束（不触发全局停机）。
  - 达到条数上限 `line_max`（来自 CLI `-n` 或配置）。
  - 收到 Stop 指令（控制总线或信号传递）。
  - 致命错误（Throw）→ 触发全局停机（极端情况，默认沿用）。
- 主组完成与进程退出：
  - 在 batch 下，acceptor 不启动，主组仅由各数据源的 picker 组成；当所有 picker 结束后，TaskManager 认为“主组完成”，进入全局收尾并退出。

## 退出判定（常驻）
- daemon 模式会启动 acceptor（网络监听、服务端组件等）；主组不会自然完成，进程保持常驻。
- 退出触发：
  - 接收 SIGTERM/SIGINT/SIGQUIT 等信号 → 优雅停机。
  - 控制总线下发 Stop 指令（企业版）。

## 行为差异与实现要点
- 采集（picker）层面的 EOF 处理：
  - EOF → `Terminate` 策略 → 仅结束当前源（返回 `Ok(false)`），不再触发全局 `stop_routine_run()`。
  - 代码：src/runtime/collector/realtime/picker.rs
- 运行模式控制 acceptor：
  - daemon：正常启动 acceptor 任务。
  - batch：跳过启动 acceptor，避免其阻塞主组完成。
  - 代码：src/orchestrator/engine/service.rs
- 主组设定与全局收尾：
  - 采集任务组（picker_group）被设为主组；当主组完成，TaskManager 执行有序下线与收尾。
  - 代码：src/orchestrator/engine/service.rs；src/runtime/actor/routine.rs

## 错误与重试策略（概览）
- 策略映射：见 src/runtime/errors/mod.rs。
  - EOF → `Terminate`（优雅结束当前源）。
  - 断线/可重试 → `FixRetry`（指数退避后继续）。
  - 数据/业务可容忍 → `Tolerant`（记录后继续）。
  - 供应商/配置等致命 → `Throw`（触发全局停机）。
- 指数退避：picker 对 `FixRetry` 会设置 backoff 窗口并继续读取，避免对短暂故障过早退出。

## 与 Sink/监控的关系
- sink/infra 组不作为主组；主组完成后，TaskManager 依序下线 sink/infra 再退出进程。
- Prometheus 等需要常驻 HTTP 服务的 sink：
  - 在 batch 模式不建议作为核心校验手段，推荐切换为文件输出。
  - 示例（已在用例中给出）：将监控 sink 的 `target` 从 `prometheus` 改为 `file`，路径 `./out/monitor.dat`。

## 用例与建议
- usecase/core 下所有用例已切换到 batch 模式，确保“读完即退”。
- 常见组合：
  - 文件源一次处理并退出：`wparse work --run-mode=batch -n 3000 --stat 2 --print_stat`
  - 服务化联调：`wparse work --run-mode=daemon --stat 5`
- 日志检查：
  - 结束前应见到每个源的“picker 正常结束”与全局“all routine group await end!”。

## 常见问题（FAQ）
- Q：为什么 batch 下不启动 acceptor？
  - A：acceptor 通常是常驻（监听网络/端口），会阻塞主组完成，导致无法“读完即退”。batch 目标是“源级完成 → 主组完成 → 进程退出”。
- Q：batch 能否限制最大处理条数？
  - A：是，使用 `-n`（line_max）。达到上限后该源结束，所有源结束即退出。
- Q：如何验证 batch 是否生效？
  - A：命令加入 `--run-mode=batch`，并观察上述关键日志；用例 `usecase/core` 全部按此模式编写。
- Q：如何在 daemon 下优雅退出？
  - A：发送 SIGTERM/SIGINT/SIGQUIT 之一；或通过控制总线下发 Stop 指令（企业版）。
