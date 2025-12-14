use wp_model_core::model::DataField;

use crate::{core::ValueProcessor, language::PipeFun};

mod base64;
mod escape;
mod net;
pub mod other;
mod time;

impl ValueProcessor for PipeFun {
    fn value_cacu(&self, in_val: DataField) -> DataField {
        match self {
            PipeFun::EnBase64(o) => o.value_cacu(in_val),
            PipeFun::DeBase64(o) => o.value_cacu(in_val),
            PipeFun::EnHtmlEscape(o) => o.value_cacu(in_val),
            PipeFun::DeHtmlEscape(o) => o.value_cacu(in_val),
            PipeFun::EnStrEscape(o) => o.value_cacu(in_val),
            PipeFun::EnJsonEscape(o) => o.value_cacu(in_val),
            PipeFun::DeJsonEscape(o) => o.value_cacu(in_val),
            PipeFun::TimeStamp(o) => o.value_cacu(in_val),
            PipeFun::TimeStampMs(o) => o.value_cacu(in_val),
            PipeFun::TimeStampUs(o) => o.value_cacu(in_val),
            PipeFun::TimeStampZone(o) => o.value_cacu(in_val),
            PipeFun::ArrGet(o) => o.value_cacu(in_val),
            PipeFun::ObjGet(o) => o.value_cacu(in_val),
            PipeFun::ToStr(o) => o.value_cacu(in_val),
            PipeFun::ToJson(o) => o.value_cacu(in_val),
            PipeFun::SkipIfEmpty(o) => o.value_cacu(in_val),
            PipeFun::Dumb(o) => o.value_cacu(in_val),
            PipeFun::SxfGet(o) => o.value_cacu(in_val),
            PipeFun::PathGet(o) => o.value_cacu(in_val),
            PipeFun::UrlGet(o) => o.value_cacu(in_val),
            PipeFun::Ip4Int(o) => o.value_cacu(in_val),
        }
    }
}
