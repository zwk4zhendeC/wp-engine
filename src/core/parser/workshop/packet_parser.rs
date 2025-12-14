//! 单个数据包解析逻辑

use super::types::ProcessResult;
use crate::core::parser::ParseOption;
use crate::core::parser::wpl_engine::pipeline::WplPipeline;
use orion_error::UvsReason;
use std::sync::Arc;
use wp_connector_api::SourceEvent;
use wp_model_core::model::data::Field;
use wpl::{WparseError, WparseReason};

/// 数据包解析器
pub struct PacketParser<'a> {
    pipelines: &'a mut Vec<WplPipeline>,
}

impl<'a> PacketParser<'a> {
    pub fn new(pipelines: &'a mut Vec<WplPipeline>) -> Self {
        Self { pipelines }
    }

    /// 处理单个事件
    pub fn parse_event(&mut self, event: &SourceEvent, setting: &ParseOption) -> ProcessResult {
        let mut max_depth = 0;
        let mut best_wpl = String::new();
        let mut best_error = None;
        let rule_cnt = self.pipelines.len();

        // 尝试用每个规则处理事件
        for (idx, wpl_line) in self.pipelines.iter_mut().enumerate() {
            let is_last = idx == rule_cnt - 1;

            // 调用 WPL 处理
            match wpl_line.proc(event, max_depth) {
                Ok((mut tdo_crate, un_parsed)) => {
                    if *setting.gen_msg_id() {
                        tdo_crate.set_id(event.event_id);
                        tdo_crate.append(Field::from_chars("wp_src_key", event.src_key.as_str()));
                        if let Some(ups_ip) = event.ups_ip {
                            tdo_crate.append(Field::from_ip("wp_src_ip", ups_ip));
                        }
                    }
                    wpl_line.hit_cnt += 1;

                    let wpl_key = wpl_line.wpl_key().to_string();
                    let record = Arc::new(tdo_crate);

                    // 根据是否有残留数据返回不同的结果
                    if un_parsed.is_empty() || un_parsed.is_empty() {
                        return ProcessResult::Success { wpl_key, record };
                    } else {
                        return ProcessResult::Partial {
                            wpl_key,
                            record,
                            residue: un_parsed.to_string(),
                        };
                    }
                }
                Err(e) => {
                    // 记录解析深度最高的错误
                    if let WparseReason::Uvs(UvsReason::DataError(_, Some(pos))) = e.reason() {
                        if *pos > max_depth {
                            max_depth = *pos;
                            best_wpl = wpl_line.wpl_key().clone();
                            best_error = Some(e.clone());
                        }
                    } else if best_error.is_none() {
                        // 如果不是 DataError，作为备选记录第一个错误
                        best_wpl = wpl_line.wpl_key().clone();
                        best_error = Some(e.clone());
                        break;
                    }

                    if is_last {
                        break;
                    }
                }
            }
        }

        // 所有规则都失败，返回深度最高的失败信息
        let best_error = best_error.unwrap_or_else(|| {
            WparseError::from(WparseReason::Uvs(UvsReason::SystemError(
                "No matching rule".to_string(),
            )))
        });
        ProcessResult::Miss(super::types::ParseFailInfo::new(
            best_wpl, best_error, max_depth,
        ))
    }
}
