use std::path::{Path, PathBuf};

use super::{Connectors, Oml, ProjectPaths, Sinks, Sources, Wpl, init::PrjScope};
use crate::{
    models::knowledge::Knowledge, sinks::clean_outputs, wparse::WParseManager, wpgen::WpGenManager,
};
use wp_conf::engine::EngineConfig;
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
    pub(super) eng_conf: Option<EngineConfig>,
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
    fn build(work_root: &Path) -> Self {
        let paths = ProjectPaths::from_root(work_root);
        let connectors = Connectors::new(paths.connectors.clone());
        let sinks_c = Sinks::new();
        let sources_c = Sources::new();
        let wpl = Wpl::new();
        let oml = Oml::new();
        let knowledge = Knowledge::new();
        let wparse_manager = WParseManager::new(work_root);
        let wpgen_manager = WpGenManager::new(work_root);

        Self {
            paths,
            eng_conf: None,
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

    /// 静态初始化：创建并初始化完整项目
    pub fn init<P: AsRef<Path>>(work_root: P, mode: PrjScope) -> RunResult<Self> {
        let mut project = Self::build(work_root.as_ref());
        project.init_components(mode)?;
        Ok(project)
    }

    /// 静态加载：基于现有结构执行校验加载
    pub fn load<P: AsRef<Path>>(work_root: P, mode: PrjScope) -> RunResult<Self> {
        let mut project = Self::build(work_root.as_ref());
        project.load_components(mode)?;
        Ok(project)
    }

    #[cfg(test)]
    pub(crate) fn bare<P: AsRef<Path>>(work_root: P) -> Self {
        Self::build(work_root.as_ref())
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

    pub(crate) fn apply_engine_paths(&mut self) {
        if let Some(conf) = &self.eng_conf {
            let sink_root = Self::resolve_with_root(self.work_root_path(), conf.sink_root());
            self.sinks_c.set_root(sink_root);
            let src_root = Self::resolve_with_root(self.work_root_path(), conf.src_root());
            self.sources_c.set_root(src_root);
        }
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
