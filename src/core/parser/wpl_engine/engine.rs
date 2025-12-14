use wp_connector_api::SourceEvent;
use wp_parse_api::{DataResult, RawData};
use wpl::WplEvaluator;

#[derive(Clone)]
pub enum ParsingEngine {
    RuleEngine(WplEvaluator),
}

impl ParsingEngine {
    pub fn proc(&self, data: &SourceEvent, oth_suc_len: usize) -> DataResult {
        match self {
            // wp-lang 的 `WplEvaluator::proc` 返回 `(DataRecord, String)`，
            // 而对外暴露的 `wp_parse_api::DataResult` 期望 `(DataRecord, RawData)`。
            // 这里做一个轻量的适配，将剩余串包装为 `RawData::String`。
            ParsingEngine::RuleEngine(vm_unit) => vm_unit
                .proc(data.payload.clone(), oth_suc_len)
                .map(|(rec, remain)| (rec, RawData::from_string(remain))),
        }
    }
}
