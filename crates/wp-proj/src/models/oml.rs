use orion_error::{ToStructError, UvsConfFrom};
use std::path::Path;
use wp_conf::utils::find_conf_files;
use wp_engine::facade::config::{WPARSE_OML_FILE, load_warp_engine_confs};
use wp_engine::facade::generator::fetch_oml_data;
use wp_error::run_error::{RunReason, RunResult};

use crate::types::CheckStatus;
use crate::utils::{config_path::ConfigPathResolver, error_handler::ErrorHandler};

#[derive(Clone)]
pub struct Oml;

impl Oml {
    pub fn new() -> Self {
        Oml
    }

    /// Initialize OML with example content for the specified project directory
    pub fn init_with_examples<P: AsRef<Path>>(work_root: P) -> RunResult<()> {
        let work_root = work_root.as_ref();
        let example_oml_content = include_str!("../example/oml/nginx.oml");
        if !example_oml_content.contains("name") || !example_oml_content.contains("rule") {
            return ErrorHandler::config_error("example OML content is missing essential fields");
        }

        Self::create_example_files(work_root)?;

        println!("OML initialized successfully with example content");
        Ok(())
    }

    /// Create example OML files in the specified project directory
    fn create_example_files(work_root: &Path) -> RunResult<()> {
        // 使用统一的路径解析器
        let oml_dir =
            ConfigPathResolver::resolve_model_path(work_root.to_string_lossy().as_ref(), "oml")?;

        // Create OML directory
        ConfigPathResolver::ensure_dir_exists(&oml_dir)?;

        // Create example OML file (single concrete file is enough for glob patterns)
        let example_oml_content = include_str!("../example/oml/nginx.oml");
        let oml_file_path = oml_dir.join("example.oml");
        ConfigPathResolver::write_file_with_dir(&oml_file_path, example_oml_content)?;

        // Create knowdb.toml file
        let knowdb_content = r#"# OML Knowledge Database Configuration
# This file defines the OML models available for use

[[models]]
name = "example_oml"
file = "example.oml"
description = "Example OML model for demonstration purposes"
rule = "/example/*"
"#;
        let knowdb_path = oml_dir.join("knowdb.toml");
        ConfigPathResolver::write_file_with_dir(&knowdb_path, knowdb_content)?;

        println!("Created example OML files:");
        println!("  - {:?}", oml_file_path);
        println!("  - {:?}", knowdb_path);

        Ok(())
    }

    pub fn check<P: AsRef<Path>>(&self, work_root: P) -> RunResult<CheckStatus> {
        let work_root = work_root.as_ref();

        // 先尝试加载配置，如果失败则继续检查 OML
        let _config_load_result = load_warp_engine_confs(work_root.to_string_lossy().as_ref());

        let oml_root =
            ConfigPathResolver::resolve_model_path(work_root.to_string_lossy().as_ref(), "oml")?;
        if !oml_root.exists() {
            return Ok(CheckStatus::Miss);
        }
        let root_str = oml_root
            .to_str()
            .ok_or_else(|| RunReason::from_conf("OML文件路径无效").to_err())?;
        let oml_files = find_conf_files(root_str, WPARSE_OML_FILE)
            .map_err(|e| RunReason::from_conf(format!("OML 查找失败: {}", e)).to_err())?;
        if oml_files.is_empty() {
            return Err(RunReason::from_conf(format!(
                "OML 文件不存在: {}/{}",
                root_str, WPARSE_OML_FILE
            ))
            .to_err());
        }
        for f in &oml_files {
            ErrorHandler::check_file_not_empty(f, "OML")?;
        }

        fetch_oml_data(root_str, WPARSE_OML_FILE)
            .map_err(|e| RunReason::from_conf(format!("parse oml failed: {}", e)).to_err())?;
        Ok(CheckStatus::Suc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_workdir;

    #[test]
    fn initialize_examples_creates_valid_files() {
        let temp = temp_workdir();
        let root = temp.path().to_str().unwrap();
        Oml::init_with_examples(root).expect("init examples");

        let example_file = temp.path().join("models/oml/example.oml");
        assert!(example_file.exists());
        assert!(!temp.path().join("models/oml/*.oml").exists());
    }
}
