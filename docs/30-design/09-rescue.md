# 救急文件结构化回放方案（Rescue Entry V1）
<!-- 角色：开发者 | 最近验证：2025-12-11 -->

**接口位置**：`src/sinks/rescue.rs`、`src/sinks/runtime/manager.rs`、`src/runtime/collector/recovery/mod.rs`。

## 背景
- 救急链路需要保证“恢复之后的数据与实时输出完全一致”。
- 为此，救急文件必须保留结构化记录，并在恢复时重新走 `SinkDataEnum::Rec/FFV` 链路，而不是依赖格式化后的文本副本。

## 设计目标
1. **保留结构化信息**：救急文件记录必须能还原出完整的 `DataRecord` 或原始字节串。
2. **恢复链路无感**：恢复流程应继续调用 `send_record`/`send_raw`，而不是写回磁盘或重新格式化。
3. **Fail-fast**：系统尚未对外发布，可以不保留旧格式，解析失败应直接报错，避免悄悄写出错误格式。
4. **实现简单**：尽量复用现有 `SinkRuntime`、`SinkTerminal`，避免在恢复路径做复杂搬运。

## 方案概述
> 核心思想：把救急文件从“格式化文本”改为“结构化 JSON 行”，恢复时读取 JSON 得到 `DataRecord` 或 `raw`，再走现有发送接口。

### 1. 救急文件写入结构化 JSON 行
- 新增 `RescueEntry` + `RescuePayload`：
  ```rust
  #[serde(tag = "kind", rename_all = "snake_case")]
  pub enum RescuePayload {
      Record { record: DataRecord },
      Raw { raw: String },
  }

  pub struct RescueEntry {
      version: u8,           // 当前版本 = 1
      #[serde(flatten)]
      payload: RescuePayload,
  }
  ```
- 每条救急数据序列化为一行 JSON，携带 `version` 字段方便后续演进。
- `RescueFileSink` 封装 JSON 序列化、写入与定期 flush（每 100 条刷新一次），保持"drop/stop 时重命名 .lock → .dat"的语义。
- `SinkRuntime::swap_back()` 改为直接构造 `RescueFileSink`，完全复用现有 route / metric 流程，不再手动 clone formatter 配置。

### 2. 恢复流程解析并回放 DataRecord
- `ActCovPicker::pick_file` 读取每一行后必须成功 `RescueEntry::parse`：
  - `Record` 变体：`send_record(ProcMeta::Null, Arc<DataRecord>)`，进入正常 formatter -> sink 路径。
  - `Raw` 变体：继续 `send_raw`，用于兼容原本走 Raw 的 sink。
- 解析失败即返回错误，直接终止恢复任务，提示管理员修复对应文件。

### 3. 测试与验证
- 单元测试：
  - `rescue_entry_roundtrip_record` 验证 `DataRecord` -> JSON -> `DataRecord` 一致。
  - `rescue_sink_writes_structured_lines` 实际写入临时文件，确认 `.lock` -> `.dat` 重命名与 JSON 存档行为。
- 运行时测试：`RUSTC_WRAPPER= cargo test rescue_entry_roundtrip_record --lib`；后续建议在集成环境跑 `usecase/core/getting_started/case_verify.sh`。

## 影响面分析
| 组件 | 影响 |
| --- | --- |
| Sink Runtime | `swap_back` 不再 clone file sink 配置，直接使用 `RescueFileSink`，简化救急切换。 |
| 恢复流程 | 默认所有 rescue `.dat` 为 JSON 行，解析失败直接报错，中断恢复并提示具体行/错误，便于快速定位。 |
| 文档/调试 | 新增设计文档（本文件），提醒一行一条 JSON。 |

## 后续优化方向
1. **批量恢复**：目前逐行读取、逐条 `send_record`，后续可在 `RescueEntry` 中标记批次或压缩，提高大批量恢复效率。
2. **扩展 payload**：可追加 `FFV`、`binary` 等 payload 类型，以覆盖更多 sink 场景。
3. **工具链支持**：wprescue CLI 可提供 `--inspect` 查看 JSON 结构，方便手工排查。

以上设计已在 `src/sinks/rescue.rs`、`src/sinks/runtime/manager.rs`、`src/runtime/collector/recovery/mod.rs` 等处实现并通过单元测试验证。
