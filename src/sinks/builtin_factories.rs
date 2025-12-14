use crate::sinks::backends::tcp::TcpFactory;
use crate::sinks::sink_build::build_file_sink;
use crate::sinks::{ASinkTestProxy, BlackHoleSink, HealthController, SyslogFactory};
use async_trait::async_trait;
//

// Built-in lightweight no-op sink implementing Async* traits
//pub struct BlackHoleSink;

struct BlackHoleFactory;

#[async_trait]
impl wp_connector_api::SinkFactory for BlackHoleFactory {
    fn kind(&self) -> &'static str {
        "blackhole"
    }
    fn validate_spec(&self, _spec: &wp_connector_api::SinkSpec) -> anyhow::Result<()> {
        // no params required
        Ok(())
    }
    async fn build(
        &self,
        _spec: &wp_connector_api::SinkSpec,
        _ctx: &wp_connector_api::SinkBuildCtx,
    ) -> anyhow::Result<wp_connector_api::SinkHandle> {
        Ok(wp_connector_api::SinkHandle::new(Box::new(
            BlackHoleSink {},
        )))
    }
}

struct FileFactory;

#[async_trait]
impl wp_connector_api::SinkFactory for FileFactory {
    fn kind(&self) -> &'static str {
        "file"
    }
    fn validate_spec(&self, spec: &wp_connector_api::SinkSpec) -> anyhow::Result<()> {
        // Require either path or base+file
        let has_path = spec.params.get("path").and_then(|v| v.as_str()).is_some();
        let has_base_file = spec.params.get("base").and_then(|v| v.as_str()).is_some()
            && spec.params.get("file").and_then(|v| v.as_str()).is_some();
        if !(has_path || has_base_file) {
            anyhow::bail!("file sink requires either 'path' or 'base'+'file'");
        }
        // Optional fmt must be a known value when provided
        if let Some(s) = spec.params.get("fmt").and_then(|v| v.as_str()) {
            let ok = matches!(
                s,
                "json" | "csv" | "show" | "kv" | "raw" | "proto" | "proto-text"
            );
            if !ok {
                anyhow::bail!(
                    "invalid fmt: '{}'; allowed: json,csv,show,kv,raw,proto,proto-text",
                    s
                );
            }
        }
        Ok(())
    }
    async fn build(
        &self,
        spec: &wp_connector_api::SinkSpec,
        ctx: &wp_connector_api::SinkBuildCtx,
    ) -> anyhow::Result<wp_connector_api::SinkHandle> {
        // Compute path from either path or base+file; when replica_cnt>1, shard filenames by idx.
        let mut path = if spec.params.contains_key("base") || spec.params.contains_key("file") {
            let base = spec
                .params
                .get("base")
                .and_then(|v| v.as_str())
                .unwrap_or("./data/out_dat");
            let file = spec
                .params
                .get("file")
                .and_then(|v| v.as_str())
                .unwrap_or("out.dat");
            std::path::Path::new(base).join(file).display().to_string()
        } else {
            spec.params
                .get("path")
                .and_then(|v| v.as_str())
                .unwrap_or("./data/out_dat/out.dat")
                .to_string()
        };
        // 可选多文件分片：当 replica_cnt>1 且显式开启时生效
        let shard_by_replica = spec
            .params
            .get("replica_shard")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        // Optional file_template for naming; evaluated when replica_cnt>1
        if ctx.replica_cnt > 1 && (shard_by_replica || spec.params.contains_key("file_template")) {
            if let Some(tpl) = spec.params.get("file_template").and_then(|v| v.as_str()) {
                // Determine parent dir and generate file name from template
                let p = std::path::Path::new(&path);
                let dir = if p.is_dir() {
                    p
                } else {
                    p.parent().unwrap_or_else(|| std::path::Path::new("."))
                };
                let fname = {
                    // 简易模板：{replica}/{replica1}/{file}
                    let mut s = String::from(tpl);
                    if s.contains("{replica}") {
                        s = s.replace("{replica}", &ctx.replica_idx.to_string());
                    }
                    if s.contains("{replica1}") {
                        s = s.replace("{replica1}", &(ctx.replica_idx + 1).to_string());
                    }
                    if s.contains("{file}") {
                        let f = p.file_name().and_then(|x| x.to_str()).unwrap_or("out.dat");
                        s = s.replace("{file}", f);
                    }
                    s
                };
                path = dir.join(fname).display().to_string();
            } else if spec.params.contains_key("base") || spec.params.contains_key("file") {
                // base+file: insert _{idx} before extension
                let p = std::path::Path::new(&path);
                let dir = p.parent().unwrap_or_else(|| std::path::Path::new("."));
                let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("out");
                let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("");
                let fname = if ext.is_empty() {
                    format!("{}_{}", stem, ctx.replica_idx)
                } else {
                    format!("{}_{}.{}", stem, ctx.replica_idx, ext)
                };
                path = dir.join(fname).display().to_string();
            }
        }
        // Formatting: default Json (align with config/docs); allow override via params.fmt = json|csv|kv|raw|proto|proto-text
        let fmt = spec
            .params
            .get("fmt")
            .and_then(|v| v.as_str())
            .map(wp_model_core::model::fmt_def::TextFmt::from)
            .unwrap_or(wp_model_core::model::fmt_def::TextFmt::Json);
        let dummy = wp_conf::structure::SinkInstanceConf::null_new(spec.name.clone(), fmt, None);
        // Build using existing file builder (AsyncFormatter<AsyncFileSink>)
        let f = build_file_sink(&dummy, &path).await?;
        Ok(wp_connector_api::SinkHandle::new(Box::new(f)))
    }
}

struct TestRescueFactory;

#[async_trait]
impl wp_connector_api::SinkFactory for TestRescueFactory {
    fn kind(&self) -> &'static str {
        "test_rescue"
    }
    fn validate_spec(&self, spec: &wp_connector_api::SinkSpec) -> anyhow::Result<()> {
        // Same as file sink
        let has_path = spec.params.get("path").and_then(|v| v.as_str()).is_some();
        let has_base_file = spec.params.get("base").and_then(|v| v.as_str()).is_some()
            && spec.params.get("file").and_then(|v| v.as_str()).is_some();
        if !(has_path || has_base_file) {
            anyhow::bail!("test_rescue requires either 'path' or 'base'+'file'");
        }
        if let Some(s) = spec.params.get("fmt").and_then(|v| v.as_str()) {
            let ok = matches!(
                s,
                "json" | "csv" | "show" | "kv" | "raw" | "proto" | "proto-text"
            );
            if !ok {
                anyhow::bail!(
                    "invalid fmt: '{}'; allowed: json,csv,show,kv,raw,proto,proto-text",
                    s
                );
            }
        }
        Ok(())
    }
    async fn build(
        &self,
        spec: &wp_connector_api::SinkSpec,
        ctx: &wp_connector_api::SinkBuildCtx,
    ) -> anyhow::Result<wp_connector_api::SinkHandle> {
        // Compute file path as in FileFactory, with replica-based shard naming
        let mut path = if spec.params.contains_key("base") || spec.params.contains_key("file") {
            let base = spec
                .params
                .get("base")
                .and_then(|v| v.as_str())
                .unwrap_or("./data/out_dat");
            let file = spec
                .params
                .get("file")
                .and_then(|v| v.as_str())
                .unwrap_or("out.dat");
            std::path::Path::new(base).join(file).display().to_string()
        } else {
            spec.params
                .get("path")
                .and_then(|v| v.as_str())
                .unwrap_or("./data/out_dat/out.dat")
                .to_string()
        };
        let shard_by_replica = spec
            .params
            .get("replica_shard")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        if ctx.replica_cnt > 1 && (shard_by_replica || spec.params.contains_key("file_template")) {
            if let Some(tpl) = spec.params.get("file_template").and_then(|v| v.as_str()) {
                let p = std::path::Path::new(&path);
                let dir = if p.is_dir() {
                    p
                } else {
                    p.parent().unwrap_or_else(|| std::path::Path::new("."))
                };
                let fname = {
                    let mut s = String::from(tpl);
                    if s.contains("{replica}") {
                        s = s.replace("{replica}", &ctx.replica_idx.to_string());
                    }
                    if s.contains("{replica1}") {
                        s = s.replace("{replica1}", &(ctx.replica_idx + 1).to_string());
                    }
                    if s.contains("{file}") {
                        let f = p.file_name().and_then(|x| x.to_str()).unwrap_or("out.dat");
                        s = s.replace("{file}", f);
                    }
                    s
                };
                path = dir.join(fname).display().to_string();
            } else if spec.params.contains_key("base") || spec.params.contains_key("file") {
                let p = std::path::Path::new(&path);
                let dir = p.parent().unwrap_or_else(|| std::path::Path::new("."));
                let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("out");
                let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("");
                let fname = if ext.is_empty() {
                    format!("{}_{}", stem, ctx.replica_idx)
                } else {
                    format!("{}_{}.{}", stem, ctx.replica_idx, ext)
                };
                path = dir.join(fname).display().to_string();
            }
        }
        let fmt = spec
            .params
            .get("fmt")
            .and_then(|v| v.as_str())
            .map(wp_model_core::model::fmt_def::TextFmt::from)
            // Default to Json to keep parity with config layer and docs
            .unwrap_or(wp_model_core::model::fmt_def::TextFmt::Json);
        let dummy = wp_conf::structure::SinkInstanceConf::null_new(spec.name.clone(), fmt, None);
        let f = build_file_sink(&dummy, &path).await?;
        let stg = HealthController::new();
        let proxy = ASinkTestProxy::new(f, stg);
        Ok(wp_connector_api::SinkHandle::new(Box::new(proxy)))
    }
}

// fast_file 工厂已移除

pub fn register_builtin_factories() {
    crate::connectors::registry::register_sink_factory(BlackHoleFactory);
    crate::connectors::registry::register_sink_factory(FileFactory);
    crate::connectors::registry::register_sink_factory(SyslogFactory);
    crate::connectors::registry::register_sink_factory(TcpFactory);
    crate::connectors::registry::register_sink_factory(TestRescueFactory);
}

#[allow(dead_code)]
pub fn make_blackhole_sink() -> Box<dyn wp_connector_api::AsyncSink> {
    Box::new(BlackHoleSink {})
}

#[cfg(test)]
mod tests {
    use super::*;
    use wp_connector_api::{AsyncRawDataSink, AsyncRecordSink, SinkFactory};

    #[tokio::test(flavor = "multi_thread")]
    async fn file_factory_supports_fmt_param() -> anyhow::Result<()> {
        let tmp = std::env::temp_dir().join(format!("wp_file_factory_fmt_{}.log", nano_ts()));
        let mut params = toml::value::Table::new();
        params.insert(
            "base".into(),
            toml::Value::String(tmp.parent().unwrap().to_string_lossy().into()),
        );
        params.insert(
            "file".into(),
            toml::Value::String(tmp.file_name().unwrap().to_string_lossy().into()),
        );
        params.insert("fmt".into(), toml::Value::String("json".into()));
        let spec = wp_connector_api::SinkSpec {
            group: String::new(),
            name: "t".into(),
            kind: "file".into(),
            connector_id: String::new(),
            params: wp_connector_api::parammap_from_toml_table(params),
            filter: None,
        };
        let ctx = wp_connector_api::SinkBuildCtx::new(std::env::current_dir().unwrap());
        let init = FileFactory.build(&spec, &ctx).await?;
        // write one record as JSON via record sink
        let mut sink = init.sink;
        let rec = wp_model_core::model::DataRecord::default();
        AsyncRecordSink::sink_record(sink.as_mut(), &rec).await?;
        AsyncRawDataSink::sink_str(sink.as_mut(), "\n").await?; // ensure newline flush
        AsyncRawDataSink::sink_str(sink.as_mut(), "").await?;
        drop(sink);
        let body = std::fs::read_to_string(tmp)?;
        assert!(body.trim_start().starts_with("{"));
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn null_factory_is_noop() -> anyhow::Result<()> {
        let spec = wp_connector_api::SinkSpec {
            group: String::new(),
            name: "n".into(),
            kind: "null".into(),
            connector_id: String::new(),
            params: wp_connector_api::parammap_from_toml_table(toml::value::Table::new()),
            filter: None,
        };
        let ctx = wp_connector_api::SinkBuildCtx::new(std::env::current_dir().unwrap());
        let init = BlackHoleFactory.build(&spec, &ctx).await?;
        let mut sink = init.sink;
        AsyncRawDataSink::sink_str(sink.as_mut(), "hello").await?;
        Ok(())
    }

    fn nano_ts() -> i128 {
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0).into()
    }
}
