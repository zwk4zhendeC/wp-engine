use std::path::{Path, PathBuf};

use super::{Connectors, Oml, ProjectPaths, Sinks, Sources, Wpl};
use crate::{
    models::knowledge::Knowledge, sinks::clean_outputs, wparse::WParseManager, wpgen::WpGenManager,
};
use wp_error::run_error::RunResult;

/// # WarpProject
///
/// 高层工程管理器，提供统一的项目管理接口。
///
/// ## 主要功能
///
/// 1. **项目初始化**: 创建完整的项目结构，包括配置、模板和模型
/// 2. **项目检查**: 验证项目配置和组件的完整性
/// 3. **组件管理**: 统一管理连接器、输入源、输出接收器等组件
/// 4. **模型管理**: 管理 WPL 解析规则和 OML 模型配置
///

pub struct WarpProject {
    // 项目路径管理器
    paths: ProjectPaths,
    // 连接器管理
    connectors: Connectors,
    // 输出接收器管理
    sinks_c: Sinks,
    // 输入源管理
    sources_c: Sources,
    // WPL 解析规则管理
    wpl: Wpl,
    // OML 模型管理
    oml: Oml,
    // 知识库管理
    knowledge: Knowledge,
    // WParse 管理器
    wparse_manager: WParseManager,
    // WPgen 管理器
    wpgen_manager: WpGenManager,
}

impl WarpProject {
    /// 创建新的项目实例
    pub fn new<P: AsRef<Path>>(work_root: P) -> Self {
        let work_root_ref = work_root.as_ref();
        let paths = ProjectPaths::from_root(work_root_ref);
        let connectors = Connectors::new(paths.connectors.clone());
        let sinks_c = Sinks::new();
        let sources_c = Sources::new();
        let wpl = Wpl::new();
        let oml = Oml::new();
        let knowledge = Knowledge::new();
        let wparse_manager = WParseManager::new(work_root_ref);
        let wpgen_manager = WpGenManager::new(work_root_ref);

        Self {
            paths,
            connectors,
            sinks_c,
            sources_c,
            wpl,
            oml,
            knowledge,
            wparse_manager,
            wpgen_manager,
        }
    }

    /// 获取工作根目录（向后兼容）
    pub fn work_root(&self) -> &str {
        self.paths.root.to_str().unwrap_or_default()
    }
    pub fn work_root_path(&self) -> &PathBuf {
        &self.paths.root
    }

    pub fn paths(&self) -> &ProjectPaths {
        &self.paths
    }

    pub fn connectors(&self) -> &Connectors {
        &self.connectors
    }

    pub fn sinks_c(&self) -> &Sinks {
        &self.sinks_c
    }

    pub fn sources_c(&self) -> &Sources {
        &self.sources_c
    }

    pub fn wpl(&self) -> &Wpl {
        &self.wpl
    }

    pub fn oml(&self) -> &Oml {
        &self.oml
    }

    pub fn knowledge(&self) -> &Knowledge {
        &self.knowledge
    }

    // ========== 配置管理方法 ==========

    /// 清理项目数据目录（委托给各个专门的模块处理）
    pub fn data_clean(&self) -> RunResult<()> {
        let mut cleaned_any = false;

        //  清理 sinks 输出数据
        if let Ok(sink_cleaned) = clean_outputs(self.work_root()) {
            cleaned_any |= sink_cleaned;
        }

        //  清理 wpgen 生成数据（委托给 WPgenManager）
        if let Ok(wpgen_cleaned) = self.wpgen_manager.clean_outputs() {
            cleaned_any |= wpgen_cleaned;
        }

        //  清理 wparse 相关临时数据（委托给 WParseManager）
        if let Ok(wparse_cleaned) = self.wparse_manager.clean_data() {
            cleaned_any |= wparse_cleaned;
        }

        if !cleaned_any {
            println!("No data files to clean");
        } else {
            println!("✓ Data cleanup completed");
        }

        Ok(())
    }
}
