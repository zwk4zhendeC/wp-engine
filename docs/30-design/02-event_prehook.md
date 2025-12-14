# DSourceEvent + EventPreHook  设计方案

本文档描述对 wp-source-api 的一次“面向并行与背压”的接口演进：引入 SourceEvent 最小数据单元与 EventPreHook 预处理钩子，将协议级预处理（例如 syslog header normalize/strip/tag 注入）从采集热路径迁移到 parse 线程中执行，从而获得随 parse 并发线性扩展的性能，同时保持对使用者零配置成本。

## 目标：
  - 把协议级“预处理”从 DataSource.recv 迁移到 parse 线程执行（天然并行，随 parse worker 扩展）。
  - DataSource 专注“取帧 + 最小标签/元信息”，去掉 recv 内的重预处理逻辑。
  - 一致的并发/背压模型：bounded channel/stream，无额外轮询。
  - 对使用者零增加成本（一个 syslog connector 即可），同时保留可调并发提示与可回退能力。

## 核心设计

### 类型：SourceEvent（源产出最小单元）

```rust
pub struct SourceEvent {
    pub src_key: String,                
    pub payload: wpl::RawData,          
    pub tags: TagSet,
    pub ups_ip: Option<std::net::IpAddr>,
    pub preproc: Option<std::sync::Arc<dyn Fn(&mut SourceEvent) + Send + Sync + 'static>>,
}
```

说明：Source 端在拆帧后只负责填充最小信息；如需协议级“规范化”（normalize/strip/tag 的派生），将处理闭包随帧下发，由 parse 线程并行调用 `preproc` 完成。

### DataSource 接口（建议·简化版）

```rust
#[async_trait]
pub trait DataSource: Send + Sync {
    async fn start(&mut self, ctrl: CtrlRx) -> SourceResult<()>;
    async fn stop(&mut self) -> SourceResult<()>;
    async fn recv(&mut self) -> SourceResult<SourceEvent>;
    fn identifier(&self) -> &str;
    fn caps(&self) -> SourceCaps { SourceCaps::default() }
}
```

说明：
- DataSource 不在 recv 内做重预处理，仅负责产出 SourceEvent；
- 协议级预处理由 SourceEvent 内的 `preproc` 闭包承担，parse 线程并行调用；
- concurrency_hint() 为引擎调度提供并发提示（可选）。

### Parse 线程集成（ActParser 侧）

伪代码：

```rust
while let Some(mut frame) = source.next().await? {
    if let Some(p) = frame.preproc.as_ref() { (p)(&mut frame); }
    let mut pkt = DataPacket::from_syslog(frame.src_key.clone(), frame.payload, frame.tags);
    if let Some(ip) = frame.ups_ip { pkt.set_ups_ip(ip); }
    // 进入 WPL 解析...
}
```

说明：预处理在 parse worker 线程内执行，天然并行，无需使用者增加 connector 数量。

## syslog 实现示意

连接/拆帧侧：
- acceptor 负责监听与连接管理；
- 每个连接 handler 将拆帧后的行转换为 SourceEvent（仅最小信息），写入 bounded mpsc；
- ActPicker 从 Source.recv() 取到 SourceEvent 并分发。

预处理侧：
- syslog 在 DataSource 内部构造一个 `Arc<dyn Fn(&mut SourceEvent)>` 的闭包（捕获 strip_header/attach_meta_tags 等配置），并在产出 SourceEvent 时将该闭包克隆到 `frame.preproc`；
- 该闭包从 frame.payload 解析 header（RFC3164/5424），按配置写入 tags 或替换 message 内容等。

说明：单连接场景也可通过内部并行策略（semaphore/线程池）限制/扩展预处理并发，但作为 v2 基线不强制要求。

## 并发与背压

- Source 内部：连接 handler → framer → bounded mpsc<SourceEvent>；
- ActPicker：next().await 无轮询；
- 预处理闭包：在 parse worker 线程同步执行（可选在闭包内部用 semaphore 控制并发）；
- WPL：保持现状；
- Sink：与本设计无直接耦合。

## 兼容性与迁移

- 保持 DataSource trait 的 `start/recv/close` 形态；
- 迁移步骤：
  1) 源端产出 `SourceEvent` 并附着 `preproc`（按需）。
  2) ActPicker 分发 `SourceEvent`；ActParser 在 `proc_frames` 中执行 `preproc`，再转换为 `DataPacket`。
  3) 其它 source（file/kafka/http）可不附带闭包（preproc=None），保持行为不变。

## 性能预期与收益

- normalize 不再串行集中在 DataSource.recv，而是随 parse 并发扩展，CPU 利用与吞吐显著提升；
- 无 per-message spawn 过度调度/乱序开销；
- 有界 channel/stream 化，背压传递与唤醒粒度更优雅；
- 对使用者零增加成本（无需复制多个 connector）。

## 参数与扩展（可选）

- concurrency_hint()：源端建议并发度（如连接数或 normalize_workers），供引擎调度参考；
- preprocessor 内部可选 semaphore：限制重预处理的并发，避免 parse worker 被单一钩子占满；
- 排序/稳定性：一般 syslog 不要求严格顺序；如需要，可为 `SourceEvent` 增加 seq_id，在 parse 端以小窗口重排（可选）。

## 风险与缓解

- 预处理耗时长：在 EventPreHook 内部增加轻量计时指标，便于定位热点；必要时加 semaphore 控制 N 并发；
- 内存峰值：通过 bounded 队列与 parse 并发控制，避免堆积；
- 行为回退：syslog 可保留“recv 内 normalize”的开关便于 A/B 回退（默认关闭）。

## 里程碑

1) API 引入：wp-source-api 增加 SourceEvent/EventPreHook/DataSource v2（已落地）。
2) 引擎改造：ActPicker 分发 SourceEvent，ActParser::proc_frames 执行 preproc 并转换 DataPacket（已落地）。
3) syslog 迁移：拆帧→SourceEvent；预处理闭包实现 normalize/strip/tag（已落地）。
4) 验证与指标：tcp2file_test 压测对比 CPU/吞吐/RSS；
5) 可选扩展：UDP 源迁移；prehook 并发控制；排序小窗口；
6) 清理遗留：移除 v1 recv 路径（在 A/B 稳定后）。

## 附：伪代码片段

```rust
// wp-source-api
pub struct SourceEvent {
    pub src_key: String,
    pub payload: wpl::RawData,
    pub tags: wp_model_core::model::TagSet,
    pub ups_ip: Option<std::net::IpAddr>,
    pub preproc: Option<std::sync::Arc<dyn Fn(&mut SourceEvent) + Send + Sync + 'static>>,
}

#[async_trait]
pub trait DataSource: Send + Sync {
    async fn start(&mut self, ctrl: CtrlRx) -> SourceResult<()>;
    async fn stop(&mut self) -> SourceResult<()>;
    async fn recv(&mut self) -> SourceResult<SourceEvent>;
    fn identifier(&self) -> &str;
    fn caps(&self) -> SourceCaps { SourceCaps::default() }
    fn concurrency_hint(&self) -> usize { 1 }
}

// ActParser 侧
loop {
    let mut frame = source.recv().await?;
    if let Some(p) = frame.preproc.as_ref() { (p)(&mut frame); }
    let mut pkt = DataPacket::from_syslog(frame.src_key.clone(), frame.payload, frame.tags);
    if let Some(ip) = frame.ups_ip { pkt.set_ups_ip(ip); }
    // do WPL parse...
}

// syslog 在产出 SourceEvent 时附带闭包（捕获 strip_header/attach_meta_tags）
let pre = {
    let strip = self.strip_header;
    let attach = self.attach_meta_tags;
    std::sync::Arc::new(move |f: &mut SourceEvent| {
        let s = match &f.payload {
            RawData::String(s) => s.as_str(),
            RawData::Bytes(b) => unsafe { std::str::from_utf8_unchecked(b) },
        };
        let norm = normalize(s);
        if attach {
            if let Some(pri) = norm.meta.pri { f.tags.set_tag("syslog.pri", pri.to_string()); }
            if let Some(ref fac) = norm.meta.facility { f.tags.set_tag("syslog.facility", fac.clone()); }
            if let Some(ref sev) = norm.meta.severity { f.tags.set_tag("syslog.severity", sev.clone()); }
        }
        if strip { f.payload = RawData::String(norm.message); }
    })
};
let frame = SourceEvent { src_key, payload, tags, ups_ip, preproc: Some(pre) };
```
