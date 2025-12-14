# DataRecord 池化方案

## 背景与目标
性能剖析显示 `_rjem_sdallocx` 与 `Vec<Field<Value>>` drop 占用了大量 CPU，主因是解析→路由→Sink 全链路的 `DataRecord` 每条都重新分配、克隆、释放。在 100W EPS 场景下，需要一个“借→用→还”的池化机制，复用 `DataRecord`/`Vec<DataField>` 内存，最大化吞吐。

## 设计概览
1. **RecordHandle**：`Arc<RecordInner>`，向下游暴露 `as_record()`、`append_tag()`、`clone()`；drop 时会把字段内存归还给所属池。
2. **RecordPool**：每个 sink worker 持有独立池（`Arc<RecordPool>`），内部维护空闲 `Vec<DataField>`；`acquire()` 借出空 `Vec`，`release()` 清理后复用。
3. **解析→路由流程**：解析阶段从本地/目标池获取 `Vec`，填充字段后直接包装为 `RecordHandle`；OML 路由中根据命中的 sink 数量使用 `Arc::make_mut` 做 copy-on-write，仅在必要时克隆。
4. **Sink 阶段**：`SinkDataEnum::Rec` 改为携带 `RecordHandle`；sink backend 仅借用数据，不负责内存释放。handle drop 后自动归还池子。

## 实施步骤
1. **实现 RecordPool/RecordHandle**
   - 结构：`RecordPool { free: SegQueue<Vec<DataField>> }`，`RecordHandle { inner: Arc<RecordInner> }`。
   - `RecordInner` 含 `Vec<DataField>` + pool 引用；`Drop` 时执行 `vec.clear()` 并 `pool.release(vec)`。

2. **解析阶段接入池**
   - `WplPipeline::proc` 中注入 `Arc<RecordPool>`；解析时 `let mut handle = pool.acquire_handle()`，填充字段后通过 `RecordHandle::into_sink_payload()` 传递给路由。
   - `ActParser`/`WplWorkshop` 初始化时带入 `Arc<RecordPool>`，各 worker 共享或使用 thread-local 池。

3. **路由阶段传递 handle**
   - `dispatcher::oml` 不再 `clone DataRecord`，改为 `RecordHandle::clone()`（copy-on-write）。追加 tags 通过 `append_tag()` 完成。
   - 命中多个 sink 时，第一份直接使用原 handle，其余通过 `Arc::clone` 获得新 handle（仅在 `append_tag` 时触发 clone）。

4. **Sink 阶段适配**
   - `SinkDataEnum::Rec` 改为携带 `RecordHandle`。
   - backend API 改为 `sink_record(&RecordHandle)` 或 `(&DataRecord)`；处理完成后 handle drop 自动归还。

5. **测试与回归**
   - 添加单元测试：池 acquire/release、handle drop 归还、多 sink copy-on-write。
   - 压测验证 `_rjem_sdallocx`、`Vec::drop` 自耗时下降。

## 后续优化方向
- 将 tags/常量字段存为 `Arc<[DataField]>`，append 时直接引用。
- 结合 thread-local pool，减少跨线程同步。
- 结合 `Bytes`/zero-copy 来进一步压缩 `DataField` 字符串分配。
