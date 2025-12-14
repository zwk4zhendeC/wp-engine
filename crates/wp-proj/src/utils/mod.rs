//! # 通用工具模块
//!
//! 提供 wproj/core 中各模块共用的工具函数和辅助类。
//!
//! ## 模块组成
//!
//! - **config_path**: 统一的配置路径解析，支持回退机制
//! - **error_handler**: 统一的错误处理策略和错误信息格式化
//! - **log_handler**: 通用的日志处理，基于 WpEngine LogConf 对象

pub mod config_path;
pub mod error_handler;
pub mod log_handler;

// Re-export 主要类型以方便使用
pub use log_handler::LogHandler;
