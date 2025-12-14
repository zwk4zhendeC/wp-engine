# ActPicker 与 Policy 设计说明（开发文档）

本文记录 runtime/collector/realtime/picker 模块的设计目标、职责划分与关键策略器（Policy）实现要点，作为后续维护与优化的基线文档。

## 设计目标

- 明确职责：将“发送（post）”与“拉取（pull）”两个行为解耦，分别受各自策略控制；调度代码只做“取计划→执行”，不散落条件判断。
- 缓冲有效：pending 作为真实缓冲池，仅在每轮最多发送一个 burst 的数据，剩余留作缓存，避免“一次清空”。
- 回退保守：post 阶段失败（Full/Closed）不应造成 CPU 自旋或 pending 失控增长；允许按水位策略决定是否 pull（或按需求切换为“最保守：本轮禁 pull”）。
- 无时间度量：不使用时间 API（Instant/sleep）影响高速路径，统一用“轮次退避 + 固定水位（LO/HI）+ burst 配额”决策。
- 结果可解释：本轮结果 RoundStat 仅反映 pull 阶段（fetch）的等待与 stop，post 不混入统计，避免歧义。

## 关键结构与职责

- ActPicker
  - 管理 pending 队列与订阅者轮转；向外暴露 `round_pick()` 与 `finish_burst_round()`。
  - 内部持有策略器：`post_policy`、`pull_policy`（跨 round 持久状态）。
  - 仅在 post 阶段调用策略更新（on_post_result），不向 RoundStat 写入；仅在 pull 后将 fetch 结果 merge 入 RoundStat。

- PostPolicy（轮次退避，不用时间）
  - 字段：
    - `burst: usize`：每轮最大发送配额（当前由 `burst_max()` 驱动，默认 16）。
    - `skip_rounds_left: u32`：剩余需要跳过 post 的轮次数。
    - `backoff_rounds: u32` / `max_backoff_rounds: u32`：退避轮次数（指数退避，默认 1→2→4→8 封顶）。
  - 接口：
    - `on_round_begin_round() -> bool`：是否跳过当前轮 post（并消耗一轮退避）。
    - `plan_post(pending) -> PostPlan { allow, batch_size }`：批大小 = min(pending, burst)。
    - `on_post_result(progressed: bool)`：进展成功→清空退避；失败（Full/Closed）→设置 `skip_rounds_left = backoff_rounds` 并翻倍 backoff。

- PullPolicy（固定水位，不用时间）
  - 字段：
    - `burst: usize`；`lo = 2×burst`；`hi = 3×burst`（均衡档位，更早停止拉取以降低 pending 高水位时长）。
  - 接口：
    - `plan_pull(pending) -> PullPlan { allow, fetch_budget }`
      - 仅当 pending < LO 时允许 pull；预算 = min(burst, HI - pending)；本轮只拉一次，且不在本轮分发。

## round_pick 执行流程（单轮）

1) 读取 `pre_pending = pending_count()`。
2) Post 阶段（发送优先）：
   - 若 `on_round_begin_round()` 返回 true：跳过 post（本轮处于退避），进入 Pull 决策。
   - 否则：
     - `plan_post(pre_pending)` 仅发送一批（最多一个 burst），调用 `handle_pending_batch(batch_size)`；
     - 调用 `on_post_result(progressed)` 更新 post 策略状态；不合并 RoundStat。
3) Pull 阶段（拉取一次，不分发）：
   - 读取 `post_pending = pending_count()`；
   - `plan_pull(post_pending)`：若 `allow=false`，本轮收尾；否则 fetch 一次并 merge 到 RoundStat；
   - 在 pull 合并后调用 `poll_cmd_now(task_ctrl)` 更新命令；返回本轮 RoundStat（仅包含 pull 的 waited/stop）。

备注：可选“最保守策略”为：post 无进展/处于退避时，本轮直接 return，禁止 pull，防止 pending 增长。当前默认语义为“post 失败仍允许按水位决定是否 pull”。

## 常量与默认

- `burst_max()`：当前固定 16（`PICKER_BURST_MAX`）；可后续按订阅者数量自适应。
- PullPolicy：`LO = 2×burst`（`PICKER_PULL_LO_MULTIPLIER`），`HI = 3×burst`（`PICKER_PULL_HI_MULTIPLIER`）。
- PostPolicy：`backoff_rounds = 1`（`PICKER_POST_BACKOFF_INITIAL_ROUNDS`），`max_backoff_rounds = 8`（`PICKER_POST_BACKOFF_MAX_ROUNDS`），增长因子 = 2（`PICKER_POST_BACKOFF_GROWTH_FACTOR`）。
- 小批合并：`PICKER_COALESCE_TRIGGER = 32`（pending 队列达到该阈值时启用合并），`PICKER_COALESCE_MAX_EVENTS = 128`（合并后最大事件数）。
- 均不使用环境变量与外部配置，避免入口膨胀与不易复现的问题。

## 与 RoundStat 的关系

- RoundStat 仅在 pull 阶段（fetch）合并：
  - `waited_for`：来自 blocking/超时等真实等待；try 模式无数据不增加等待；
  - `need_stop`：在 pull 合并后调用 `poll_cmd_now` 更新；
  - `proc_cnt`：仅统计 pull 阶段合入的处理量（当前 fetch_into_pending 不增加 proc，通常为 0）。
- post 阶段不写 RoundStat，避免混淆“发送”与“拉取”的结果口径。

## 典型场景

- pending ≥ LO：缓存充足 → 本轮不 pull；只消化 pending（最多一个 burst）。
- post 被 Full/Closed 阻塞：
  - 默认：允许依据水位决定是否 pull；如 pending < LO 则允许一次补水（本轮不分发）。
  - 最保守（可切换）：本轮直接收尾，禁止 pull，防止 pending 增长。
- try_receive 模式下：允许一次非阻塞尝试；若源返回 None，不会增加 pending。
- 命令处理：在 pull 合并后调用 `poll_cmd_now` 一次，命令优先可及时生效。

## 测试要点

- 策略器（policy.rs）应覆盖：
  - PostPolicy：退避轮次递增与清零；plan_post 的批大小
  - PullPolicy：LO/HI 水位约束与预算计算
- round_pick 行为：
  - post 成功：RoundStat 不增加；pending 减少至 `burst`；本轮不拉取
  - post 失败（Full/Closed）：允许按水位 pull；try 模式只尝试不累加；或最保守禁 pull
  - blocked + Stop：need_stop 置位，pull 是否尝试由策略与水位决定
- run_dispatch_loop：
  - max_line 正常退出；Stop 命令在超时内退出；pending 阻塞时避免过度 fetch；EOF 快速退出

## 后续可选优化

- 自适应 burst / LO/HI：按订阅者数、历史发送成功率动态调整（仍建议保留无配置入口）。
- 指标埋点：pending_depth、blocked_rounds、try_reads_per_round、post_cooldown_hits，便于观测与调参。
- 分片策略（可选）：在 handle_pending_batch 保持“整批成功/失败”语义前提下，引入按 slice 发送（例如等于 burst），进一步精细化缓冲释放。
