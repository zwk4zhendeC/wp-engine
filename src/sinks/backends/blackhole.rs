use async_trait::async_trait;
use wp_connector_api::SinkResult;

#[derive(Clone)]
pub struct BlackHoleSink {}

#[async_trait]
impl wp_connector_api::AsyncCtrl for BlackHoleSink {
    async fn stop(&mut self) -> SinkResult<()> {
        Ok(())
    }
    async fn reconnect(&mut self) -> SinkResult<()> {
        Ok(())
    }
}

#[async_trait]
impl wp_connector_api::AsyncRecordSink for BlackHoleSink {
    async fn sink_record(&mut self, _data: &wp_model_core::model::DataRecord) -> SinkResult<()> {
        Ok(())
    }

    async fn sink_records(
        &mut self,
        _data: Vec<std::sync::Arc<wp_model_core::model::DataRecord>>,
    ) -> SinkResult<()> {
        Ok(())
    }
}

#[async_trait]
impl wp_connector_api::AsyncRawDataSink for BlackHoleSink {
    async fn sink_str(&mut self, _data: &str) -> SinkResult<()> {
        Ok(())
    }
    async fn sink_bytes(&mut self, _data: &[u8]) -> SinkResult<()> {
        Ok(())
    }

    async fn sink_str_batch(&mut self, _data: Vec<&str>) -> SinkResult<()> {
        Ok(())
    }

    async fn sink_bytes_batch(&mut self, _data: Vec<&[u8]>) -> SinkResult<()> {
        Ok(())
    }
}
