use crate::common::io_locate::find_connectors_base_dir as resolve_base;
use orion_conf::TomlIO;
use orion_conf::error::{ConfIOReason, OrionConfResult};
use orion_error::UvsValidationFrom;
use orion_error::{ErrorOwe, ErrorWith, ToStructError};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::types::{SourceConnector, SrcConnectorFileRec};

/// 自任意起点向上寻找 `connectors/source.d` 并返回其绝对路径（不再支持旧布局）
pub fn find_connectors_dir(start: &Path) -> Option<PathBuf> {
    resolve_base(start, "source.d")
}

/// Legacy alias retained for CLI compatibility
pub fn resolve_connectors_base_dir(start: &Path) -> Option<PathBuf> {
    find_connectors_dir(start)
}

/// 加载 `connectors/source.d` 下的全部连接器（去重校验 id）
pub fn load_connectors_for(start: &Path) -> OrionConfResult<BTreeMap<String, SourceConnector>> {
    let base = find_connectors_dir(start);
    let mut files: Vec<PathBuf> = Vec::new();
    if let Some(dir) = base {
        let mut entries: Vec<_> = fs::read_dir(&dir)
            .owe_conf()
            .with(&dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().map(|s| s == "toml").unwrap_or(false))
            .collect();
        entries.sort();
        files.extend(entries);
    }
    let mut map = BTreeMap::new();
    for fp in files {
        let cf: SrcConnectorFileRec = SrcConnectorFileRec::load_toml(&fp)?;
        for c in cf.connectors {
            if map.contains_key(&c.id) {
                return ConfIOReason::from_validation(format!(
                    "duplicate connector id '{}' (file {})",
                    c.id,
                    fp.display()
                ))
                .err_result();
            }
            map.insert(c.id.clone(), c);
        }
    }
    Ok(map)
}
