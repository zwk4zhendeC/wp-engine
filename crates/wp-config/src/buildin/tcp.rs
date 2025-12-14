use educe::Educe;
use orion_conf::error::{ConfIOReason, OrionConfResult};
use orion_error::{ToStructError, UvsValidationFrom};

/// 与 docs/80-reference/params/sink_tcp.md 对齐的 TCP Sink 配置建模。
#[derive(Educe, Serialize, Deserialize, PartialEq, Clone)]
#[educe(Debug, Default)]
#[allow(dead_code)]
pub struct TcpSinkConf {
    #[educe(Default = "127.0.0.1")]
    pub(crate) addr: String,
    #[educe(Default = 9000)]
    pub(crate) port: usize,
    #[educe(Default = "line")]
    pub(crate) framing: String, // "line" | "len"
    /// 发送队列感知 backoff（可选；缺省由上层根据速率开关控制）
    pub max_backoff: Option<bool>,
}

#[allow(dead_code)]
impl TcpSinkConf {
    pub fn addr_str(&self) -> String {
        format!("{}:{}", self.addr, self.port)
    }
}

impl crate::structure::Validate for TcpSinkConf {
    fn validate(&self) -> OrionConfResult<()> {
        if self.addr.trim().is_empty() {
            return ConfIOReason::from_validation("tcp.addr must not be empty").err_result();
        }
        if self.port == 0 || self.port > 65535 {
            return ConfIOReason::from_validation("tcp.port must be in 1..=65535").err_result();
        }
        let f = self.framing.to_ascii_lowercase();
        if f != "line" && f != "len" {
            return ConfIOReason::from_validation("tcp.framing must be 'line' or 'len'")
                .err_result();
        }
        Ok(())
    }
}
