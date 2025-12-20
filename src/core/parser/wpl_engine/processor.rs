//! 批量处理逻辑

use super::types::{ParsedDatSet, ProcessResult};
use crate::core::parser::{ParseOption, WplEngine};
use crate::sinks::{ProcMeta, SinkPackage, SinkRecUnit};
use std::collections::HashMap;
use wp_connector_api::SourceEvent;
use wpl::WparseError;

impl WplEngine {
    /// 解析并分组处理后的数据
    pub fn batch_parse_package(
        &mut self,
        batch: Vec<SourceEvent>,
        setting: &ParseOption,
    ) -> Result<ParsedDatSet, WparseError> {
        let mut sink_groups: HashMap<String, SinkPackage> = HashMap::new();
        let mut residue_data = Vec::new();
        let mut miss_packets = Vec::new();

        debug_data!("Processing events: len={}", batch.len());
        // 处理每个数据包
        for data in batch {
            match self.pipelines.parse_event(&data, setting) {
                ProcessResult::Success { wpl_key, record } => {
                    // 完全成功解析
                    let rec_unit = SinkRecUnit::new(data.event_id, ProcMeta::Null, record);
                    sink_groups.entry(wpl_key).or_default().push(rec_unit);
                }
                ProcessResult::Partial {
                    wpl_key,
                    record,
                    residue,
                } => {
                    // 部分成功，有残留数据
                    let rec_unit = SinkRecUnit::new(data.event_id, ProcMeta::Null, record);
                    sink_groups.entry(wpl_key).or_default().push(rec_unit);
                    residue_data.push((data.event_id, residue));
                }
                ProcessResult::Miss(fail_info) => {
                    // 完全失败，记录深度最高的错误信息
                    miss_packets.push((data, fail_info));
                }
            }
        }

        Ok(ParsedDatSet {
            sink_groups,
            residue_data,
            missed_packets: miss_packets,
        })
    }
}
