use getset::WithSetters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SrcConnectorFileRec {
    #[serde(default)]
    pub connectors: Vec<SourceConnector>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SourceConnector {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub allow_override: Vec<String>,
    #[serde(default)]
    pub params: toml::value::Table,
}

// V2 [[sources]] 项（应用层 SourceConfigParser 的配置数据结构迁移至配置层，便于统一装配）
#[derive(Debug, Clone, Deserialize, Serialize, WithSetters)]
pub struct WpSource {
    pub key: String,
    #[set_with = "pub"]
    #[serde(default)]
    pub enable: Option<bool>,
    pub connect: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default, rename = "params", alias = "params_override")]
    pub params: toml::value::Table,
}

/// Deprecated alias: maintained for crates that still refer to `SourceItem`
pub type SourceItem = WpSource;

/// V2 源配置包装器：`[[sources]]` 列表
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WpSourcesConfig {
    #[serde(default)]
    pub sources: Vec<WpSource>,
}

/// Legacy alias for compatibility with tooling referencing `WarpSources`
pub type WarpSources = WpSourcesConfig;
