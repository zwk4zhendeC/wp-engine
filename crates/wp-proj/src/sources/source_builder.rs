use wp_conf::sources::types::SourceItem;

/// V2SourceItem 的 Builder 模式
pub struct SourceItemBuilder {
    key: String,
    connect: String,
    enable: Option<bool>,
    tags: Vec<String>,
    params: toml::value::Table,
}

#[allow(dead_code)]
impl SourceItemBuilder {
    pub fn new(key: &str, connect: &str) -> Self {
        Self {
            key: key.to_string(),
            connect: connect.to_string(),
            enable: Some(true),
            tags: vec![],
            params: toml::value::Table::new(),
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enable = Some(enabled);
        self
    }

    pub fn tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    pub fn tags(mut self, tags: Vec<&str>) -> Self {
        self.tags.extend(tags.into_iter().map(|t| t.to_string()));
        self
    }

    pub fn param(mut self, key: &str, value: toml::Value) -> Self {
        self.params.insert(key.to_string(), value);
        self
    }

    pub fn param_str(mut self, key: &str, value: &str) -> Self {
        self.params
            .insert(key.to_string(), toml::Value::String(value.to_string()));
        self
    }

    pub fn param_int(mut self, key: &str, value: i64) -> Self {
        self.params
            .insert(key.to_string(), toml::Value::Integer(value));
        self
    }

    pub fn build(self) -> SourceItem {
        SourceItem {
            key: self.key,
            enable: self.enable,
            connect: self.connect,
            tags: self.tags,
            params: self.params,
        }
    }
}

/// 常用的 source 便利构造函数
pub mod source_builders {
    use super::*;

    /// 创建文件读取源
    pub fn file_source(key: &str, file: &str) -> SourceItem {
        SourceItemBuilder::new(key, "file_src")
            .param_str("file", file)
            .param_str("encode", "text")
            .build()
    }

    /// 创建 UDP syslog 接收源
    pub fn syslog_udp_source(key: &str, addr: &str, port: i64) -> SourceItem {
        SourceItemBuilder::new(key, "syslog_udp_src")
            .param_str("addr", addr)
            .param_int("port", port)
            .param_str("protocol", "udp")
            .build()
    }

    /// 创建 TCP syslog 接收源
    #[allow(dead_code)]
    pub fn syslog_tcp_source(key: &str, addr: &str, port: i64) -> SourceItem {
        SourceItemBuilder::new(key, "syslog_tcp_src")
            .param_str("addr", addr)
            .param_int("port", port)
            .param_str("protocol", "tcp")
            .build()
    }

    /// 通用的 V2SourceItem 构建器
    #[allow(dead_code)]
    pub fn builder(key: &str, connect: &str) -> SourceItemBuilder {
        SourceItemBuilder::new(key, connect)
    }
}

#[cfg(test)]
mod tests {
    use super::source_builders;

    #[test]
    fn file_source_sets_expected_fields() {
        let item = source_builders::file_source("file_1", "gen.dat");
        assert_eq!(item.key, "file_1");
        assert_eq!(item.connect, "file_src");
        assert_eq!(item.params.get("file").unwrap().as_str(), Some("gen.dat"));
    }

    #[test]
    fn syslog_builder_respects_protocol() {
        let item = source_builders::syslog_udp_source("syslog_1", "0.0.0.0", 9000);
        assert_eq!(item.params.get("protocol").unwrap().as_str(), Some("udp"));
        assert_eq!(item.params.get("port").unwrap().as_integer(), Some(9000));
    }
}
