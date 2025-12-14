//! Syslog 源常量与类型别名（统一复用 tcp::framing 定义）

use wp_data_model::tags::parse_tags;

pub use crate::sources::tcp::framing::{DEFAULT_TCP_RECV_BYTES, Message, STOP_CHANNEL_CAPACITY};

/// Extract tags from `Vec<String>` items (k:v / k=v / flag)
#[allow(dead_code)]
pub fn tags_from_vec(items: &[String]) -> wp_model_core::model::TagSet {
    parse_tags(items)
}
