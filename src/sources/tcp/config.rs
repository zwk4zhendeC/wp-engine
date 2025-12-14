use super::framing::{DEFAULT_TCP_RECV_BYTES, FramingMode};

#[derive(Debug, Clone)]
pub struct TcpConf {
    pub addr: String,
    pub port: u16,
    pub tcp_recv_bytes: usize,
    pub framing: FramingMode,
    pub instances: usize,
}

pub const DEFAULT_TCP_SOURCE_INSTANCES: usize = 1;
pub const MAX_TCP_SOURCE_INSTANCES: usize = 16;

impl TcpConf {
    pub fn from_params(params: &wp_connector_api::ParamMap) -> anyhow::Result<Self> {
        let addr = params
            .get("addr")
            .and_then(|v| v.as_str())
            .unwrap_or("0.0.0.0")
            .to_string();
        let port = params.get("port").and_then(|v| v.as_i64()).unwrap_or(9000) as u16;
        let tcp_recv_bytes = params
            .get("tcp_recv_bytes")
            .and_then(|v| v.as_i64())
            .filter(|&v| v > 0)
            .unwrap_or(DEFAULT_TCP_RECV_BYTES as i64) as usize;
        let framing = match params
            .get("framing")
            .and_then(|v| v.as_str())
            .unwrap_or("auto")
            .to_ascii_lowercase()
            .as_str()
        {
            "line" => FramingMode::Line,
            "len" | "length" => FramingMode::Len,
            _ => {
                let prefer_newline = params
                    .get("prefer_newline")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                FramingMode::Auto { prefer_newline }
            }
        };

        let instances = params
            .get("instances")
            .and_then(|v| v.as_i64())
            .unwrap_or(DEFAULT_TCP_SOURCE_INSTANCES as i64);
        anyhow::ensure!(
            instances >= 1,
            "tcp.instances must be between 1 and {}",
            MAX_TCP_SOURCE_INSTANCES
        );
        anyhow::ensure!(
            instances as usize <= MAX_TCP_SOURCE_INSTANCES,
            "tcp.instances must be between 1 and {}",
            MAX_TCP_SOURCE_INSTANCES
        );
        let instances = instances as usize;

        Ok(Self {
            addr,
            port,
            tcp_recv_bytes,
            framing,
            instances,
        })
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.addr, self.port)
    }
}
