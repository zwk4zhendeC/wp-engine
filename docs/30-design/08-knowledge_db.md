# ADR: KnowDB 运行形态取舍（Authority DB vs In‑Memory）
<!-- 角色：开发者 | 最近验证：2025-12-11 -->

Status: Accepted
Date: 2025-10-23
Related: crates/wp-knowledge (loader/facade), docs/10-user/02-config/knowdb_config.md
Decision Owner: wp-knowledge 维护者

## 背景
- 运行模式：一次性载入 CSV→SQLite，运行期无修改，多线程并发只读。
- 问题：是否必须落地“权威库（authority.sqlite）”？还是可直接使用纯内存模式？

## 选项
- 线程克隆（ThreadClonedMDB，默认小数据）
  - 路径：models/knowledge/knowdb.toml → 构建 authority.sqlite → 每线程备份到内存连接（只读）
  - 优点：每线程零锁竞争，p99 延迟低；快照稳定、可复现
  - 代价：内存占用 ≈ DB 大小 × 线程数；每线程首次访问有一次 backup 成本
- WAL + 只读连接池（MemDB::new_file + WAL，默认中/大数据）
  - 路径：models/knowledge/knowdb.toml → 构建 authority.sqlite（开启 WAL）→ 只读连接池并发查询
  - 优点：单份文件由 OS 缓存，多线程共享，不放大内存；吞吐稳定
  - 代价：相对线程内存副本，极端 p99 略高；需要合理设置 pool_size
- 纯内存模式（不落地文件，非默认）
  - 路径：models/knowledge/knowdb.toml → 直接导入到内存库（MemDB::instance/shared_pool）→ 设置 Provider
  - 优点：无文件落地；部署简化
  - 风险：
    - shared_pool 依赖 SQLite 的共享缓存特性（平台/构建相关）
    - 崩溃后需重载；跨进程共享不可用；不可作为可分发快照

## 决策
- 保留并默认使用“权威库”形态：
  - 小数据或低延迟优先：线程克隆（facade::init_thread_cloned_from_knowdb）
  - 中/大数据或线程数大、内存敏感：WAL + 只读连接池（loader::build_authority_from_knowdb → facade::init_wal_pool_from_authority）
- 在明确不想落地文件的场景下，允许“纯内存模式”作为可选方案（适用于测试、一次性任务或资源严格受限的运行环境）。

## 取舍与适用性
- 线程克隆（ThreadClonedMDB）
  - 适用：数据体量小（例如 ≤ 100MB）、线程数中等（≤ 8–16）、追求低延迟
  - 代价：内存占用随线程线性放大；线程首次备份冷启动抖动
- WAL + 池（只读）
  - 适用：数据体量中/大或线程数较多、内存紧张、需要在稳定吞吐下扩展
  - 代价：极端 p99 较线程内存副本略高
- 纯内存（不落地文件）
  - 适用：测试、一次性离线脚本、对可复现与跨进程无要求的场景
  - 代价：不可跨进程共享；崩溃后需要重载；平台依赖（shared-cache）

## 实现与接口（当前）

**接口位置**：`crates/wp-knowledge/src/facade.rs`、`crates/wp-knowledge/src/loader.rs`。

- 线程克隆（默认小数据）
  - `facade::init_thread_cloned_from_knowdb(root, knowdb_toml, authority_uri)` - 构建权威库并初始化线程克隆 Provider
  - `facade::init_thread_cloned_from_authority(authority_uri)` - 直接使用已有权威库初始化
- WAL + 池（默认中/大数据）
  - `loader::build_authority_from_knowdb(...)` 构建权威库后，
  - `facade::init_wal_pool_from_authority(authority_uri, pool_size)` 初始化 WAL + 连接池模式
- 内存模式（MemDB）
  - `facade::init_mem_provider(memdb)` - 使用 MemDB 作为 Provider

**查询门面**：
```rust
pub trait QueryFacade: Send + Sync {
    fn query_row(&self, sql: &str) -> KnowledgeResult<Vec<DataField>>;
    fn query_named(&self, sql: &str, params: &[(&str, &dyn ToSql)]) -> KnowledgeResult<Vec<DataField>>;
    fn query_cipher(&self, table: &str) -> KnowledgeResult<Vec<String>>;
}

// 全局查询接口
pub fn query_row(sql: &str) -> KnowledgeResult<Vec<DataField>>;
pub fn query_named(sql: &str, params: &[(&str, &dyn ToSql)]) -> KnowledgeResult<Vec<DataField>>;
pub fn query_cipher(table: &str) -> KnowledgeResult<Vec<String>>;
pub fn cache_query<N>(...) -> Vec<DataField>;  // 带缓存的查询
```

## 结论
- 当前默认策略：保留并优先使用“权威库”文件；依据数据体量与资源选择线程克隆或 WAL+池。
- 纯内存模式不作为默认，但允许作为特定场景的轻量方案。

## 迁移与运维提示
- 回滚/复现：以 authority.sqlite 为快照，版本切换/回滚更安全；
- 线程克隆：线程数量波动大时，备份开销可能放大，建议改用 WAL+池；
- 池配置：`pool_size ≈ min(线程数, 2×CPU核数)` 为经验值，可视化观察等待与 p95 决定调整。
