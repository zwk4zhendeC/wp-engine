use orion_error::{ToStructError, UvsConfFrom};
use std::path::{Path, PathBuf};
use wp_engine::facade::config::{WPARSE_RULE_FILE, load_warp_engine_confs};
use wp_error::run_error::{RunReason, RunResult};
use wpl::WplCode;

use crate::utils::{config_path::ConfigPathResolver, error_handler::ErrorHandler};

#[derive(Clone)]
pub struct Wpl;

impl Wpl {
    pub fn new() -> Self {
        Wpl
    }

    /// Initialize WPL with example content for the specified project directory
    pub fn init_with_examples<P: AsRef<Path>>(work_root: P) -> RunResult<()> {
        let work_root = work_root.as_ref();
        // Include example WPL content using include_str!
        let example_wpl_content = include_str!("../example/wpl/nginx/parse.wpl");

        // Parse the example WPL content to validate it
        let code = WplCode::build(
            PathBuf::from("example/nginx/parse.wpl"),
            example_wpl_content,
        )
        .map_err(|e| RunReason::from_conf(format!("build example wpl failed: {}", e)).to_err())?;

        let _pkg = code.parse_pkg().map_err(|e| {
            RunReason::from_conf(format!("parse example wpl failed: {}", e)).to_err()
        })?;

        // Create WPL directory and example files
        Self::create_example_files(work_root)?;

        println!("WPL initialized successfully with example content and sample data");
        Ok(())
    }

    /// Create example WPL files in the specified project directory
    fn create_example_files(work_root: &Path) -> RunResult<()> {
        // 使用统一的路径解析器
        let wpl_dir =
            ConfigPathResolver::resolve_model_path(work_root.to_string_lossy().as_ref(), "wpl")?;

        // Create WPL directory
        ConfigPathResolver::ensure_dir_exists(&wpl_dir)?;

        // Create example WPL file
        let example_wpl_content = include_str!("../example/wpl/nginx/parse.wpl");
        let parse_wpl_path = wpl_dir.join("parse.wpl");
        ConfigPathResolver::write_file_with_dir(&parse_wpl_path, example_wpl_content)?;

        // Create sample data file
        let sample_data = Self::get_sample_data();
        let sample_data_path = wpl_dir.join("sample.dat");
        ConfigPathResolver::write_file_with_dir(&sample_data_path, sample_data)?;

        println!("Created example WPL files:");
        println!("  - {:?}", parse_wpl_path);
        println!("  - {:?}", sample_data_path);

        Ok(())
    }

    /// Get the sample data content as a string
    pub fn get_sample_data() -> &'static str {
        include_str!("../example/wpl/nginx/sample.dat")
    }

    pub fn check<P: AsRef<Path>>(&self, work_root: P) -> RunResult<()> {
        let work_root = work_root.as_ref();
        let (_cm, main) = load_warp_engine_confs(work_root.to_string_lossy().as_ref())?;

        let rules =
            wp_conf::utils::find_conf_files(main.rule_root(), WPARSE_RULE_FILE).unwrap_or_default();

        // 如果没有找到规则文件，尝试手动查找 *.wpl 文件
        if rules.is_empty() {
            let rule_root = main.rule_root();

            // 需要获取绝对路径来搜索文件
            let absolute_rule_root = if std::path::Path::new(rule_root).is_absolute() {
                rule_root.to_string()
            } else {
                // 相对路径需要与work_root组合
                work_root.join(rule_root).to_string_lossy().to_string()
            };

            let wpl_pattern = format!("{}/*.wpl", absolute_rule_root);

            if let Ok(glob_results) = glob::glob(&wpl_pattern) {
                let wpl_files: Vec<_> = glob_results.filter_map(Result::ok).collect();

                if !wpl_files.is_empty() {
                    // 使用找到的 .wpl 文件
                    for fp in wpl_files {
                        let raw = std::fs::read_to_string(&fp).unwrap_or_default();
                        if raw.trim().is_empty() {
                            return ErrorHandler::config_error(format!(
                                "配置错误: WPL文件为空: {:?}",
                                fp
                            ));
                        }
                        let code = WplCode::build(fp.clone(), raw.as_str()).map_err(|e| {
                            RunReason::from_conf(format!("build wpl failed: {:?}: {}", fp, e))
                                .to_err()
                        })?;
                        let _pkg = code.parse_pkg().map_err(|e| {
                            RunReason::from_conf(format!("parse wpl failed: {:?}: {}", fp, e))
                                .to_err()
                        })?;
                    }
                    return Ok(());
                }
            }
        }

        // 检查是否有任何WPL规则文件存在
        if rules.is_empty() {
            return ErrorHandler::config_error("配置错误: 未找到任何WPL规则文件 (*.wpl)");
        }

        for fp in rules {
            let raw = std::fs::read_to_string(&fp).unwrap_or_default();
            if raw.trim().is_empty() {
                return ErrorHandler::config_error(format!("配置错误: WPL文件为空: {:?}", fp));
            }
            let code = WplCode::build(fp.clone(), raw.as_str()).map_err(|e| {
                RunReason::from_conf(format!("build wpl failed: {:?}: {}", fp, e)).to_err()
            })?;
            let _pkg = code.parse_pkg().map_err(|e| {
                RunReason::from_conf(format!("parse wpl failed: {:?}: {}", fp, e)).to_err()
            })?;
        }
        Ok(())
    }
}
