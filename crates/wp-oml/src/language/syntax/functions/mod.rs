pub mod pipe;
pub mod time;
use std::fmt::{Display, Formatter};

use derive_getters::Getters;
#[derive(strum_macros::Display, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BuiltinFunction {
    #[strum(to_string = "Time::now")]
    Now(FunNow),
    #[strum(to_string = "Time::now_date")]
    NowDate(FunNowDate),
    #[strum(to_string = "Time::now_time")]
    NowTime(FunNowTime),
    #[strum(to_string = "Time::now_hour")]
    NowHour(FunNowHour),
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, PartialEq)]
pub struct FunOperation {
    fun: BuiltinFunction,
}
impl FunOperation {
    pub fn new(fun: BuiltinFunction) -> Self {
        Self { fun }
    }
}
impl Display for FunOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}() ", self.fun)
    }
}

pub use pipe::{
    EncodeType, PIPE_ARR_GET, PIPE_BASE64_DE, PIPE_BASE64_EN, PIPE_HTML_ESCAPE_DE,
    PIPE_HTML_ESCAPE_EN, PIPE_JSON_ESCAPE_DE, PIPE_JSON_ESCAPE_EN, PIPE_OBJ_GET, PIPE_PATH_GET,
    PIPE_SKIP_IF_EMPTY, PIPE_STR_ESCAPE_EN, PIPE_SXF_GET, PIPE_TIMESTAMP, PIPE_TIMESTAMP_MS,
    PIPE_TIMESTAMP_US, PIPE_TIMESTAMP_ZONE, PIPE_TO_JSON, PIPE_TO_STRING, PIPE_URL_GET, PathType,
    PipeArrGet, PipeBase64Decode, PipeBase64Encode, PipeFun, PipeHtmlEscapeDecode,
    PipeHtmlEscapeEncode, PipeJsonEscapeDE, PipeJsonEscapeEN, PipeObjGet, PipePathGet,
    PipeSkipIfEmpty, PipeStrEscapeEN, PipeSxfGet, PipeTimeStamp, PipeTimeStampMS, PipeTimeStampUS,
    PipeTimeStampZone, PipeToJson, PipeToString, PipeUrlGet, TimeStampUnit, UrlType,
};
pub use time::*;
