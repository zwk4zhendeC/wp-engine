//! Syslog source factory implementation
//!
//! This module provides the factory for creating syslog data sources
//! with support for both UDP and TCP protocols.

use super::config::SyslogConfig;
use super::tcp_source::TcpSyslogSource;
use super::udp_source::UdpSyslogSource;
use crate::sources::tcp::{FramingMode, TcpAcceptor, TcpSource};
use orion_conf::UvsConfFrom;
use orion_error::ToStructError;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use wp_conf::limits::tcp_reader_batch_channel_cap;
use wp_connector_api::SourceReason;
use wp_connector_api::{
    AcceptorHandle, SourceBuildCtx, SourceFactory, SourceHandle, SourceMeta, SourceResult,
    SourceSvcIns,
};
use wp_data_model::tags::parse_tags;
use wp_model_core::model::TagSet;

/// Syslog source factory that creates both UDP and TCP syslog sources
pub struct SyslogSourceFactory {}

impl SyslogSourceFactory {
    /// Create a new syslog source factory
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for SyslogSourceFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl SourceFactory for SyslogSourceFactory {
    fn kind(&self) -> &'static str {
        "syslog"
    }

    async fn build(
        &self,
        spec: &wp_connector_api::SourceSpec,
        _ctx: &SourceBuildCtx,
    ) -> SourceResult<SourceSvcIns> {
        let fut = async {
            if let Some(p) = spec.params.get("protocol").and_then(|v| v.as_str()) {
                let p = p.to_ascii_lowercase();
                if p != "udp" && p != "tcp" {
                    anyhow::bail!("invalid protocol: {} (must be 'udp' or 'tcp')", p);
                }
            }
            if let Some(v) = spec.params.get("tcp_recv_bytes").and_then(|v| v.as_i64())
                && v <= 0
            {
                anyhow::bail!("tcp_recv_bytes must be > 0 (got {})", v);
            }
            if let Some(v) = spec.params.get("port").and_then(|v| v.as_i64())
                && !(0..=65535).contains(&v)
            {
                anyhow::bail!("port out of range: {} (allow 0 or 1..=65535)", v);
            }

            let config = SyslogConfig::from_params(&spec.params);
            let mut tags = TagSet::default();
            tags.set_tag("access_source", "syslog".to_string());
            tags.set_tag("syslog_protocol", format!("{:?}", config.protocol));
            let parsed_tags = parse_tags(&spec.tags);
            for (key, value) in parsed_tags.item {
                tags.set_tag(&key, value);
            }

            let meta_builder = |tagset: &TagSet| -> SourceMeta {
                let mut meta = SourceMeta::new(spec.name.clone(), spec.kind.clone());
                for (k, v) in tagset.item.iter() {
                    meta.tags.set(k.clone(), v.clone());
                }
                meta
            };

            let svc = match config.protocol {
                super::config::Protocol::Udp => {
                    let meta = meta_builder(&tags);
                    let source = UdpSyslogSource::new(
                        spec.name.clone(),
                        config.address(),
                        tags,
                        config.strip_header,
                        config.attach_meta_tags,
                        config.fast_strip,
                    )
                    .await?;
                    SourceSvcIns::new()
                        .with_sources(vec![SourceHandle::new(Box::new(source), meta)])
                }
                super::config::Protocol::Tcp => {
                    let pool = Arc::new(Mutex::new(HashSet::<u64>::new()));
                    let (reg_tx, reg_rx) = mpsc::channel(tcp_reader_batch_channel_cap());
                    let framing = FramingMode::Auto {
                        prefer_newline: config.prefer_newline,
                    };

                    let inner = TcpSource::new(
                        spec.name.clone(),
                        tags.clone(),
                        config.address(),
                        config.tcp_recv_bytes,
                        framing,
                        pool.clone(),
                        reg_rx,
                    )?;
                    let acceptor = TcpAcceptor::new(
                        spec.name.clone(),
                        config.address(),
                        1000,
                        pool,
                        vec![reg_tx],
                    );

                    let meta = meta_builder(&tags);
                    let syslog = TcpSyslogSource::new(
                        spec.name.clone(),
                        tags,
                        config.strip_header,
                        config.attach_meta_tags,
                        config.fast_strip,
                        inner,
                    )
                    .await?;

                    SourceSvcIns::new()
                        .with_sources(vec![SourceHandle::new(Box::new(syslog), meta)])
                        .with_acceptor(AcceptorHandle::new(spec.name.clone(), Box::new(acceptor)))
                }
            };

            Ok(svc)
        };

        fut.await
            .map_err(|e: anyhow::Error| SourceReason::from_conf(e.to_string()).to_err())
    }
}
