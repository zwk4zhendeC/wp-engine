//! Engine-side wrappers for source/sink factory registries.
//! These forward to the registries defined in wp-connector-api to avoid
//! dependency cycles from config crates back to engine.

use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::panic::Location;
use std::sync::{Arc, RwLock};
use wp_connector_api::{SinkFactory, SourceFactory};

type SinkRec = (Arc<dyn SinkFactory>, &'static Location<'static>);
type SrcRec = (Arc<dyn SourceFactory>, &'static Location<'static>);
type SinkReg = RwLock<HashMap<String, SinkRec>>;
type SrcReg = RwLock<HashMap<String, SrcRec>>;
static SINKS: OnceCell<SinkReg> = OnceCell::new();
static SRCS: OnceCell<SrcReg> = OnceCell::new();

fn sink_reg() -> &'static SinkReg {
    SINKS.get_or_init(|| RwLock::new(HashMap::new()))
}
fn src_reg() -> &'static SrcReg {
    SRCS.get_or_init(|| RwLock::new(HashMap::new()))
}

// ---------- Sink ----------
#[track_caller]
pub fn register_sink_factory<F: SinkFactory>(f: F) {
    let kind = f.kind().to_string();
    let arc: Arc<dyn SinkFactory> = Arc::new(f);
    if let Ok(mut w) = sink_reg().write() {
        w.insert(kind, (arc, Location::caller()));
    }
}
#[track_caller]
pub fn register_sink_arc(kind: &str, f: Arc<dyn SinkFactory>) {
    if let Ok(mut w) = sink_reg().write() {
        w.insert(kind.to_string(), (f, Location::caller()));
    }
}
pub fn get_sink_factory(kind: &str) -> Option<Arc<dyn SinkFactory>> {
    sink_reg()
        .read()
        .ok()
        .and_then(|r| r.get(kind).map(|(f, _)| f.clone()))
}
pub fn list_sink_kinds() -> Vec<String> {
    sink_reg()
        .read()
        .ok()
        .map(|r| r.keys().cloned().collect())
        .unwrap_or_default()
}

// ---------- Source ----------
#[track_caller]
pub fn register_source_factory<F: SourceFactory>(f: F) {
    let kind = f.kind().to_string();
    let arc: Arc<dyn SourceFactory> = Arc::new(f);
    if let Ok(mut w) = src_reg().write() {
        w.insert(kind, (arc, Location::caller()));
    }
}
#[track_caller]
pub fn register_source_arc(kind: &str, f: Arc<dyn SourceFactory>) {
    if let Ok(mut w) = src_reg().write() {
        w.insert(kind.to_string(), (f, Location::caller()));
    }
}
pub fn get_source_factory(kind: &str) -> Option<Arc<dyn SourceFactory>> {
    src_reg()
        .read()
        .ok()
        .and_then(|r| r.get(kind).map(|(f, _)| f.clone()))
}
pub fn list_source_kinds() -> Vec<String> {
    src_reg()
        .read()
        .ok()
        .map(|r| r.keys().cloned().collect())
        .unwrap_or_default()
}

pub fn sink_diagnostics() -> Vec<(String, &'static Location<'static>)> {
    sink_reg()
        .read()
        .ok()
        .map(|r| r.iter().map(|(k, (_f, loc))| (k.clone(), *loc)).collect())
        .unwrap_or_default()
}

pub fn source_diagnostics() -> Vec<(String, &'static Location<'static>)> {
    src_reg()
        .read()
        .ok()
        .map(|r| r.iter().map(|(k, (_f, loc))| (k.clone(), *loc)).collect())
        .unwrap_or_default()
}

// ---------- Import from API registries (for compatibility) ----------
// Note: no compatibility import layer. Runtime only recognizes engine registry now.
