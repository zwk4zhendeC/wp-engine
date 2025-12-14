//! WplWorkshop 实现

use super::WplWorkshop;
use crate::core::parser::ParseOption;
use crate::stat::MonSend;
use wp_connector_api::SourceEvent;
use wpl::WparseResult;

use super::batch_processor::BatchProcessor;
use super::data_sender::DataSender;

impl WplWorkshop {
    /// 批量处理多个数据包，提高性能
    /// 通过批量聚合数据到相同 sink 来减少网络/IO 开销
    pub async fn proc_batch(
        &mut self,
        batch: Vec<SourceEvent>,
        setting: &ParseOption,
    ) -> WparseResult<()> {
        if batch.is_empty() {
            return Ok(());
        }
        // 第一阶段：解析并分组数据
        let mut processor = BatchProcessor::new(&mut self.pipelines);
        let parsed_data = processor.batch_parse_package(batch, setting)?;

        // 第二阶段：发送数据（跳过逻辑在 DataSender 内部细分：仅屏蔽业务 sinks，保留 infra 通道）
        let sender = DataSender::new(self);
        sender.send_batched_data(parsed_data).await
    }

    /// 处理单个事件（向后兼容接口）
    pub async fn proc_async(&self, event: SourceEvent, setting: &ParseOption) -> WparseResult<()> {
        // 将单个事件包装成批次处理
        let batch = vec![event];
        let mut workshop = WplWorkshop {
            pipelines: self.pipelines.clone(),
            conveyor: self.conveyor.clone(),
        };
        workshop.proc_batch(batch, setting).await
    }
}

impl WplWorkshop {
    /// 发送统计数据
    pub async fn send_stat(&mut self, mon_send: &MonSend) -> WparseResult<()> {
        for i in self.pipelines.iter_mut() {
            i.send_stat(mon_send).await?;
        }
        Ok(())
    }
}
