pub(crate) mod indexing;
pub(crate) mod setting;
pub(crate) mod workshop;
// Rename local wpl module to avoid confusion with external crate `wpl`
//pub(crate) mod plg_pipes;
pub(crate) mod wpl_engine;

pub(crate) use indexing::ResourceIndexer;
pub(crate) use setting::ParseOption;
pub(crate) use workshop::WplWorkshop;
// Re-export local implementations to keep public API aligned with internal types
pub(crate) use self::wpl_engine::engine::ParsingEngine;
pub(crate) use self::wpl_engine::pipeline::WplPipeline;
pub(crate) use self::wpl_engine::repo::{SpaceIndex, WplRepository};
