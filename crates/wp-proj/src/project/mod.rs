// Project management: 项目管理模块（统一管理项目相关的所有功能）
pub mod check;
pub mod checker;
pub mod init;
//pub mod summary;
pub mod tests;
pub mod warp;

// Re-export for backward compatibility - now from their new modules
pub use super::connectors::{Connectors, ProjectPaths};
pub use super::models::{Oml, Wpl};
pub use super::sinks::Sinks;
pub use super::sources::Sources;
pub use check::check_types::{Cell, ConnectorCounts, Row, SourceBreakdown};
pub use checker::{CheckComponent, CheckComponents, CheckOptions};
pub use warp::WarpProject;
