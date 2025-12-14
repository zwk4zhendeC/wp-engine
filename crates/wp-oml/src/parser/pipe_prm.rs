use std::str::FromStr;

use crate::language::{
    EncodeType, PIPE_ARR_GET, PIPE_BASE64_DE, PIPE_HTML_ESCAPE_DE, PIPE_HTML_ESCAPE_EN,
    PIPE_JSON_ESCAPE_DE, PIPE_JSON_ESCAPE_EN, PIPE_OBJ_GET, PIPE_PATH_GET, PIPE_SKIP_IF_EMPTY,
    PIPE_STR_ESCAPE_EN, PIPE_SXF_GET, PIPE_TIMESTAMP, PIPE_TIMESTAMP_MS, PIPE_TIMESTAMP_US,
    PIPE_TIMESTAMP_ZONE, PIPE_TO_JSON, PIPE_URL_GET, PathType, PipeArrGet, PipeHtmlEscapeDecode,
    PipeHtmlEscapeEncode, PipeJsonEscapeDE, PipeJsonEscapeEN, PipeObjGet, PipePathGet,
    PipeSkipIfEmpty, PipeStrEscapeEN, PipeSxfGet, PipeTimeStamp, PipeTimeStampMS, PipeTimeStampUS,
    PipeTimeStampZone, PipeToJson, PipeUrlGet, PreciseEvaluator, TimeStampUnit, UrlType,
};
use crate::language::{PIPE_BASE64_EN, PIPE_TO_STRING, PipeBase64Decode};
use crate::language::{
    PIPE_IP4_INT, PiPeOperation, PipeBase64Encode, PipeFun, PipeIp4Int, PipeToString,
};
use crate::parser::keyword::kw_gw_pipe;
use crate::parser::oml_aggregate::oml_var_get;
use crate::winnow::error::ParserError;
use winnow::ascii::{alphanumeric0, digit1, multispace0};
use winnow::combinator::{alt, fail, opt, repeat};
use winnow::error::{ContextError, ErrMode};
use winnow::stream::Stream; // for checkpoint/reset on &str
use wp_parser::Parser;
use wp_parser::WResult;
use wp_parser::fun::fun_trait::{Fun1Builder, Fun2Builder};
use wp_parser::fun::parser;
use wp_parser::symbol::symbol_pipe;
use wpl::parser::utils::take_key;

impl Fun1Builder for PipeArrGet {
    type ARG1 = usize;
    fn args1(data: &mut &str) -> WResult<Self::ARG1> {
        multispace0.parse_next(data)?;
        let index = digit1.parse_next(data)?;
        let i: usize = index.parse::<usize>().unwrap_or(0);
        Ok(i)
    }

    fn fun_name() -> &'static str {
        PIPE_ARR_GET
    }

    fn build(args: Self::ARG1) -> Self {
        PipeArrGet { index: args }
    }
}
impl Fun2Builder for PipeTimeStampZone {
    type ARG1 = i32;
    type ARG2 = TimeStampUnit;
    fn fun_name() -> &'static str {
        PIPE_TIMESTAMP_ZONE
    }
    fn args1(data: &mut &str) -> WResult<i32> {
        let sign = opt("-").parse_next(data)?;
        multispace0.parse_next(data)?;
        let zone = digit1.parse_next(data)?;
        let i: i32 = zone.parse::<i32>().unwrap_or(0);
        if sign.is_some() { Ok(-i) } else { Ok(i) }
    }
    fn args2(data: &mut &str) -> WResult<TimeStampUnit> {
        let unit = alt((
            "ms".map(|_| TimeStampUnit::MS),
            "us".map(|_| TimeStampUnit::US),
            "ss".map(|_| TimeStampUnit::SS),
            "s".map(|_| TimeStampUnit::SS),
        ))
        .parse_next(data)?;
        Ok(unit)
    }
    fn build(args: (i32, TimeStampUnit)) -> PipeTimeStampZone {
        PipeTimeStampZone {
            zone: args.0,
            unit: args.1,
        }
    }
}
impl Fun1Builder for PipeObjGet {
    type ARG1 = String;
    fn args1(data: &mut &str) -> WResult<Self::ARG1> {
        multispace0.parse_next(data)?;
        let name = take_key(data)?;
        Ok(name.to_string())
    }

    fn fun_name() -> &'static str {
        PIPE_OBJ_GET
    }

    fn build(args: Self::ARG1) -> Self {
        PipeObjGet { name: args }
    }
}
impl Fun1Builder for PipeBase64Decode {
    type ARG1 = EncodeType;

    fn args1(data: &mut &str) -> WResult<Self::ARG1> {
        multispace0.parse_next(data)?;
        let val: &str = alphanumeric0::<&str, ErrMode<ContextError>>
            .parse_next(data)
            .unwrap();
        if val.is_empty() {
            Ok(EncodeType::Utf8)
        } else {
            Ok(EncodeType::from_str(val).map_err(|e| {
                warn_data!("unimplemented format {} base64 decode: {}", val, e);
                ErrMode::<ContextError>::from_input(data)
            })?)
        }
    }

    fn fun_name() -> &'static str {
        PIPE_BASE64_DE
    }

    fn build(args: Self::ARG1) -> Self {
        PipeBase64Decode { encode: args }
    }
}
impl Fun1Builder for PipeSxfGet {
    type ARG1 = String;
    fn args1(data: &mut &str) -> WResult<Self::ARG1> {
        multispace0.parse_next(data)?;
        let val: &str = alphanumeric0::<&str, ErrMode<ContextError>>
            .parse_next(data)
            .unwrap();
        Ok(val.to_string())
    }

    fn fun_name() -> &'static str {
        PIPE_SXF_GET
    }

    fn build(args: Self::ARG1) -> Self {
        PipeSxfGet { key: args }
    }
}
impl Fun1Builder for PipePathGet {
    type ARG1 = PathType;
    fn args1(data: &mut &str) -> WResult<Self::ARG1> {
        multispace0.parse_next(data)?;
        let val: &str = alphanumeric0::<&str, ErrMode<ContextError>>
            .parse_next(data)
            .unwrap();

        if val.is_empty() {
            Ok(PathType::Default)
        } else {
            Ok(PathType::from_str(val).map_err(|e| {
                warn_data!("invalid path_get arg '{}': {}", val, e);
                ErrMode::<ContextError>::from_input(data)
            })?)
        }
    }

    fn fun_name() -> &'static str {
        PIPE_PATH_GET
    }

    fn build(args: Self::ARG1) -> Self {
        PipePathGet { key: args }
    }
}
impl Fun1Builder for PipeUrlGet {
    type ARG1 = UrlType;
    fn args1(data: &mut &str) -> WResult<Self::ARG1> {
        multispace0.parse_next(data)?;
        let val: &str = alphanumeric0::<&str, ErrMode<ContextError>>
            .parse_next(data)
            .unwrap();

        if val.is_empty() {
            Ok(UrlType::Default)
        } else {
            Ok(UrlType::from_str(val).map_err(|e| {
                warn_data!("invalid url_get arg '{}': {}", val, e);
                ErrMode::<ContextError>::from_input(data)
            })?)
        }
    }

    fn fun_name() -> &'static str {
        PIPE_URL_GET
    }

    fn build(args: Self::ARG1) -> Self {
        PipeUrlGet { key: args }
    }
}
pub fn oml_aga_pipe(data: &mut &str) -> WResult<PreciseEvaluator> {
    kw_gw_pipe.parse_next(data)?;
    let from = oml_var_get.parse_next(data)?;
    let items = repeat(1.., oml_pipe).parse_next(data)?;
    Ok(PreciseEvaluator::Pipe(PiPeOperation::new(from, items)))
}

// 支持省略前缀 `pipe` 的管道表达式：read(...) | func | func ...
pub fn oml_aga_pipe_noprefix(data: &mut &str) -> WResult<PreciseEvaluator> {
    let cp = data.checkpoint();
    let from = oml_var_get.parse_next(data)?;
    match repeat(1.., oml_pipe).parse_next(data) {
        Ok(items) => Ok(PreciseEvaluator::Pipe(PiPeOperation::new(from, items))),
        Err(_e) => {
            data.reset(&cp);
            fail.parse_next(data)
        }
    }
}

pub fn oml_pipe(data: &mut &str) -> WResult<PipeFun> {
    symbol_pipe.parse_next(data)?;
    multispace0.parse_next(data)?;
    let fun = alt((
        parser::call_fun_args2::<PipeTimeStampZone>.map(PipeFun::TimeStampZone),
        parser::call_fun_args1::<PipeArrGet>.map(PipeFun::ArrGet),
        parser::call_fun_args1::<PipeObjGet>.map(PipeFun::ObjGet),
        parser::call_fun_args1::<PipeBase64Decode>.map(PipeFun::DeBase64),
        parser::call_fun_args1::<PipeSxfGet>.map(PipeFun::SxfGet),
        parser::call_fun_args1::<PipePathGet>.map(PipeFun::PathGet),
        parser::call_fun_args1::<PipeUrlGet>.map(PipeFun::UrlGet),
        PIPE_HTML_ESCAPE_EN.map(|_| PipeFun::EnHtmlEscape(PipeHtmlEscapeEncode::default())),
        PIPE_HTML_ESCAPE_DE.map(|_| PipeFun::DeHtmlEscape(PipeHtmlEscapeDecode::default())),
        PIPE_STR_ESCAPE_EN.map(|_| PipeFun::EnStrEscape(PipeStrEscapeEN::default())),
        PIPE_JSON_ESCAPE_EN.map(|_| PipeFun::EnJsonEscape(PipeJsonEscapeEN::default())),
        PIPE_JSON_ESCAPE_DE.map(|_| PipeFun::DeJsonEscape(PipeJsonEscapeDE::default())),
        PIPE_BASE64_EN.map(|_| PipeFun::EnBase64(PipeBase64Encode::default())),
        PIPE_TIMESTAMP_MS.map(|_| PipeFun::TimeStampMs(PipeTimeStampMS::default())),
        PIPE_TIMESTAMP_US.map(|_| PipeFun::TimeStampUs(PipeTimeStampUS::default())),
        PIPE_TIMESTAMP.map(|_| PipeFun::TimeStamp(PipeTimeStamp::default())),
        PIPE_TO_JSON.map(|_| PipeFun::ToJson(PipeToJson::default())),
        PIPE_TO_STRING.map(|_| PipeFun::ToStr(PipeToString::default())),
        PIPE_SKIP_IF_EMPTY.map(|_| PipeFun::SkipIfEmpty(PipeSkipIfEmpty::default())),
        PIPE_IP4_INT.map(|_| PipeFun::Ip4Int(PipeIp4Int::default())),
    ))
    .parse_next(data)?;
    Ok(fun)
}

#[cfg(test)]
mod tests {
    use crate::parser::pipe_prm::oml_aga_pipe;
    use crate::parser::utils::for_test::assert_oml_parse;
    use wp_parser::WResult;

    #[test]
    fn test_oml_crate_lib() -> WResult<()> {
        let mut code = r#" pipe take(ip) | to_string | to_json | base64_en | base64_de(Utf8)"#;
        assert_oml_parse(&mut code, oml_aga_pipe);

        let mut code =
            r#" pipe take(ip) | to_string | html_escape_en | html_escape_de | str_escape_en"#;
        assert_oml_parse(&mut code, oml_aga_pipe);

        let mut code = r#" pipe take(ip) | to_string | json_escape_en | json_escape_de"#;
        assert_oml_parse(&mut code, oml_aga_pipe);

        let mut code = r#" pipe take(ip) | to_timestamp | to_timestamp_ms | to_timestamp_us"#;
        assert_oml_parse(&mut code, oml_aga_pipe);

        let mut code = r#" pipe take(ip) | to_timestamp_zone(8,ms) | to_timestamp_zone(-8,ss)"#;
        assert_oml_parse(&mut code, oml_aga_pipe);

        let mut code = r#" pipe take(ip) | skip_if_empty"#;
        assert_oml_parse(&mut code, oml_aga_pipe);

        let mut code = r#" pipe take(ip) | sxf_get(xx)"#;
        assert_oml_parse(&mut code, oml_aga_pipe);

        let mut code = r#" pipe take(ip) | path_get(name)"#;
        assert_oml_parse(&mut code, oml_aga_pipe);

        let mut code = r#" pipe take(ip) | url_get(host)"#;
        assert_oml_parse(&mut code, oml_aga_pipe);
        Ok(())
    }
}
