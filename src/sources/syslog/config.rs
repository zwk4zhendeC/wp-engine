//! Configuration structures for syslog sources

use super::constants::DEFAULT_TCP_RECV_BYTES;

/// Configuration for syslog sources
#[derive(Debug, Clone)]
pub struct SyslogConfig {
    pub addr: String,
    pub port: u16,
    pub protocol: Protocol,
    pub tcp_recv_bytes: usize,
    pub strip_header: bool,
    pub attach_meta_tags: bool,
    pub fast_strip: bool,
    /// Prefer newline framing first (then octet-counting). Default: false
    pub prefer_newline: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Udp,
    Tcp,
}

impl SyslogConfig {
    /// Parse configuration directly from params table (Factory path)
    pub fn from_params(params: &wp_connector_api::ParamMap) -> Self {
        let addr = params
            .get("addr")
            .and_then(|v| v.as_str())
            .unwrap_or("0.0.0.0")
            .to_string();
        let port = params.get("port").and_then(|v| v.as_i64()).unwrap_or(514) as u16;
        let protocol = params
            .get("protocol")
            .and_then(|v| v.as_str())
            .unwrap_or("UDP");
        let protocol = match protocol.to_uppercase().as_str() {
            "TCP" => Protocol::Tcp,
            _ => Protocol::Udp,
        };
        let tcp_recv_bytes = params
            .get("tcp_recv_bytes")
            .and_then(|v| v.as_i64())
            .filter(|&v| v > 0) // Only accept positive values
            .unwrap_or(DEFAULT_TCP_RECV_BYTES as i64) as usize;
        // Tri-state external flag: `header_mode` controls strip/attach
        // - keep  => strip=false, attach=false
        // - strip => strip=true,  attach=false
        // - parse => strip=true,  attach=true
        let header_mode = params
            .get("header_mode")
            .and_then(|v| v.as_str())
            .unwrap_or("parse")
            .to_ascii_lowercase();
        let (strip_header, attach_meta_tags) = match header_mode.as_str() {
            "keep" => (false, false),
            "strip" => (true, false),
            "parse" => (true, true),
            other => {
                // Fallback to parse, but make error obvious in logs
                log::warn!(
                    "syslog.header_mode invalid: '{}', fallback to 'parse'",
                    other
                );
                (true, true)
            }
        };
        let fast_strip = params
            .get("fast_strip")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let prefer_newline = params
            .get("prefer_newline")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        Self {
            addr,
            port,
            protocol,
            tcp_recv_bytes,
            strip_header,
            attach_meta_tags,
            fast_strip,
            prefer_newline,
        }
    }

    /// Get the full address string
    pub fn address(&self) -> String {
        format!("{}:{}", self.addr, self.port)
    }
}
