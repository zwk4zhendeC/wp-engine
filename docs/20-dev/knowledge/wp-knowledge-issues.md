# WP-Knowledge 评审问题清单（实现层）

更新时间：2025-10-23
范围：`crates/wp-knowledge`（MemDB、ThreadClonedMDB、facade 与配置加载）

## 高优先级
- SQL 注入风险（表名拼接）
  - 位置：`mem/memdb.rs:242-246`、`mem/thread_clone.rs:180-184`
  - 建议：表名白名单/标识符校验或安全引用（仅允许 `[A-Za-z0-9_]`）；只允许访问配置定义过的表。

- 门面层 `cache_query` 吞错
  - 位置：`facade.rs:164-166`
  - 建议：记录 error 日志，或新增策略开关；与 `memdb.rs` 中 `cache_query` 的日志对齐。

- CSV 导入未使用事务
  - 位置：`mem/memdb.rs:366-520`
  - 建议：启用 `BEGIN/COMMIT` 与可配置批大小，失败回滚。

- 列名缓存无界
  - 位置：`mem/memdb.rs:45-50`
  - 建议：替换为 LRU（限容量），或提供禁用开关。

- 行映射代码重复
  - 位置：`mem/memdb.rs:150-184,206-240` 与 `mem/thread_clone.rs:47-76,103-126`
  - 建议：抽取公共函数，统一 ValueRef->DataField 映射。

- 线程克隆缺少刷新机制
  - 位置：`mem/thread_clone.rs`
  - 建议：提供受控 reload（清理 TLS 连接）、或版本化权威库切换（限测试/工具特性）。

## 中优先级
- API 命名易混淆
  - `query_row/query_row_params` 仅返回“首行的列数组”，非“多行”；需在文档/注释中明确，必要时提供 `query_rows`。

- 路径与目录健壮性
  - 位置：`facade::init_thread_cloned_from_paths`
  - 建议：创建父目录；异常路径日志化。

- ToSql/FromSql 映射边界
  - Blob 以 lossy UTF-8 映射为文本；若未来需要二进制，考虑扩展 DataField 支持。

## 配置层（temp_db.toml）
- 一致性错误：`example_1` 与 SQL 表名不一致。
- 语义建议：`scope` → `expected_rows = { min, max }`；`enabled` 开关；`columns.by_header` 优先。
- 外置 SQL：支持 `sql.dir` 与 `{table}` 占位符；`insert_auto.columns` 降低重复。
- CSV 方言/错误策略：`csv` 与 `load = { transaction, batch_size, on_error }`。

## 迁移建议（简版）
1. 修正示例文件的一致性问题；
2. 代码优先加载外置 SQL（若缺失再用内联/自动生成）；
3. `table_load` 增加事务、批大小与错误策略；
4. 为 `query_cipher` 增加表名白名单；
5. 替换列名缓存为 LRU；抽取行映射公共逻辑。

