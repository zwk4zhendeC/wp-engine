use crate::language::prelude::*;

pub mod base64;
pub mod escape;
pub mod fmt;
pub mod net;
pub mod other;
pub mod time;
pub use base64::*;
pub use escape::*;
pub use fmt::*;
pub use net::*;
pub use other::*;
pub use time::*;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum PipeFun {
    EnBase64(PipeBase64Encode),
    DeBase64(PipeBase64Decode),
    EnHtmlEscape(PipeHtmlEscapeEncode),
    DeHtmlEscape(PipeHtmlEscapeDecode),
    EnStrEscape(PipeStrEscapeEN),
    EnJsonEscape(PipeJsonEscapeEN),
    DeJsonEscape(PipeJsonEscapeDE),
    TimeStamp(PipeTimeStamp),
    TimeStampMs(PipeTimeStampMS),
    TimeStampUs(PipeTimeStampUS),
    TimeStampZone(PipeTimeStampZone),
    ArrGet(PipeArrGet),
    ObjGet(PipeObjGet),
    ToStr(PipeToString),
    ToJson(PipeToJson),
    SkipIfEmpty(PipeSkipIfEmpty),
    Dumb(PipeDumb),
    SxfGet(PipeSxfGet),
    PathGet(PipePathGet),
    UrlGet(PipeUrlGet),
    Ip4Int(PipeIp4Int),
}

impl Display for PipeFun {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PipeFun::EnBase64(_) => write!(f, "{}", PIPE_BASE64_EN),
            PipeFun::DeBase64(v) => write!(f, "{}", v),
            PipeFun::EnHtmlEscape(_) => write!(f, "{}", PIPE_HTML_ESCAPE_EN),
            PipeFun::EnStrEscape(_) => write!(f, "{}", PIPE_STR_ESCAPE_EN),
            PipeFun::EnJsonEscape(_) => write!(f, "{}", PIPE_JSON_ESCAPE_EN),
            PipeFun::DeJsonEscape(_) => write!(f, "{}", PIPE_JSON_ESCAPE_DE),
            PipeFun::DeHtmlEscape(_) => write!(f, "{}", PIPE_HTML_ESCAPE_DE),
            PipeFun::TimeStamp(_) => write!(f, "{}", PIPE_TIMESTAMP),
            PipeFun::TimeStampMs(_) => write!(f, "{}", PIPE_TIMESTAMP_MS),
            PipeFun::TimeStampUs(_) => write!(f, "{}", PIPE_TIMESTAMP_US),
            PipeFun::TimeStampZone(v) => write!(f, "{}", v),
            PipeFun::ArrGet(v) => write!(f, "{}", v),
            PipeFun::ObjGet(v) => write!(f, "{}", v),
            PipeFun::ToJson(_) => write!(f, "{}", PIPE_TO_JSON),
            PipeFun::ToStr(_) => write!(f, "{}", PIPE_TO_STRING),
            PipeFun::SkipIfEmpty(_) => write!(f, "{}", PIPE_SKIP_IF_EMPTY),
            PipeFun::Dumb(_) => write!(f, "{}", PIPE_TO_STRING),
            PipeFun::SxfGet(v) => write!(f, "{}", v),
            PipeFun::PathGet(v) => write!(f, "{}", v),
            PipeFun::UrlGet(v) => write!(f, "{}", v),
            PipeFun::Ip4Int(v) => write!(f, "{}", v),
        }
    }
}

impl Default for PipeFun {
    fn default() -> Self {
        PipeFun::Dumb(PipeDumb::default())
    }
}
