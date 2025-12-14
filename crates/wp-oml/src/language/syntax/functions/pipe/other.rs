use crate::core::ValueProcessor;
use crate::language::prelude::*;
use strum_macros::EnumString;

use wp_parser::fun::fun_trait::Fun1Builder;

pub const PIPE_TO_STRING: &str = "to_string";
#[derive(Default, Builder, Debug, Clone, Getters, Serialize, Deserialize)]
pub struct PipeToString {}

pub const PIPE_ARR_GET: &str = "arr_get";
#[derive(Clone, Debug, Default, Builder)]
pub struct PipeArrGet {
    pub(crate) index: usize,
}
impl Display for PipeArrGet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", Self::fun_name(), self.index)
    }
}

pub const PIPE_SKIP_IF_EMPTY: &str = "skip_if_empty";
#[derive(Clone, Debug, Default)]
pub struct PipeSkipIfEmpty {}

pub const PIPE_OBJ_GET: &str = "obj_get";
#[derive(Clone, Debug, Default)]
pub struct PipeObjGet {
    pub(crate) name: String,
}

impl Display for PipeObjGet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", Self::fun_name(), self.name)
    }
}

pub const PIPE_SXF_GET: &str = "sxf_get";
#[derive(Default, Debug, Clone)]
pub struct PipeSxfGet {
    pub key: String,
}

impl Display for PipeSxfGet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", PIPE_SXF_GET, self.key)
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, EnumString, strum_macros::Display)]
pub enum PathType {
    #[default]
    Default,
    #[strum(serialize = "name")]
    FileName,
    #[strum(serialize = "path")]
    Path,
}
pub const PIPE_PATH_GET: &str = "path_get";
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PipePathGet {
    pub key: PathType,
}

impl Display for PipePathGet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", PIPE_PATH_GET, self.key)
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, EnumString, strum_macros::Display)]
pub enum UrlType {
    #[default]
    Default,
    /// 获取域名部分
    #[strum(serialize = "domain")]
    Domain,
    /// 获取完整的 HTTP 请求主机（包含端口）
    #[strum(serialize = "host")]
    HttpReqHost,
    /// 获取 HTTP 请求 URI（包含路径和查询参数）
    #[strum(serialize = "uri")]
    HttpReqUri,
    /// 获取 HTTP 请求路径
    #[strum(serialize = "path")]
    HttpReqPath,
    /// 获取 HTTP 请求查询参数
    #[strum(serialize = "params")]
    HttpReqParams,
}

pub const PIPE_URL_GET: &str = "url_get";
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PipeUrlGet {
    pub key: UrlType,
}

impl Display for PipeUrlGet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", PIPE_URL_GET, self.key)
    }
}

#[derive(Default, Builder, Debug, Clone, Getters, Serialize, Deserialize)]
pub struct PipeDumb {}
impl Display for PipeDumb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", PIPE_TO_STRING)
    }
}
impl ValueProcessor for PipeDumb {
    fn value_cacu(&self, _in_val: DataField) -> DataField {
        todo!()
    }
}
