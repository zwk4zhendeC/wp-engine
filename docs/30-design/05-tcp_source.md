# TCP Source 设计

## 目标
1. 允许一个 connector 根据配置生成多个 `DataSource` 实例，由上层统一注册调度。
2. 将 Acceptor 与 Source 解耦，拆分 `AcceptorHandle`，便于共享/独立管理。
3. 代码结构清晰：`SourceInstance` 只负责数据读取，Acceptor 只负责接入。

## 新接口草案
```
pub struct SourceInstance {
    pub source: Box<dyn DataSource>,
    pub metadata: SourceMeta,
}

pub struct AcceptorHandle {
    pub name: String,
    pub acceptor: Box<dyn Acceptor>,
}

pub struct BuildResult {
    pub sources: Vec<SourceInstance>,
    pub acceptor: Option<AcceptorHandle>,
}

#[async_trait]
pub trait SourceFactory {
    fn kind(&self) -> &'static str;
    fn validate_spec(&self, spec: &ResolvedSourceSpec) -> anyhow::Result<()>;
    async fn build(&self, spec: &ResolvedSourceSpec, ctx: &SourceBuildCtx)
        -> anyhow::Result<BuildResult>;
}
```

- `sources`：一个 connector 可生成多个 `SourceInstance`（例如根据 `instances` 配置），上层遍历注册。
- `acceptor`：可选，供需要监听/分发连接的 connector（如 TCP）使用；若不需要可返回 `None`。
- `SourceMeta` 可包含 name/kind/tags 等元信息，用于后续统计/调试。

## 收益
- 多实例 `TcpSource` 的实现可直接复用接口，无需额外工具。
- Acceptor 生命周期与 Source 解耦，便于监控和故障处理。
- 上层 orchestrator 注册过程更清晰，为后续引入更复杂的 backpressure/调度策略打下基础。
