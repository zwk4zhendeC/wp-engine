use crate::ast::WplFun;
use crate::ast::processor::{
    PFCharsExists, PFCharsIn, PFCharsNotExists, PFDigitExists, PFDigitIn, PFFdExists, PFIpAddrIn,
    StubFun,
};
use crate::eval::runtime::field_pipe::{DFPipeProcessor, FieldIndex};
use winnow::combinator::fail;
use wp_model_core::model::{DataField, Value};
use wp_parser::Parser;
use wp_parser::WResult;
use wp_parser::symbol::ctx_desc;

impl DFPipeProcessor for PFCharsExists {
    #[inline]
    fn process(&self, value: &mut Vec<DataField>, index: Option<&FieldIndex>) -> WResult<()> {
        let target = self.target.as_str();
        if let Some(ix) = index
            && let Some(i) = ix.get(target)
        {
            if let Some(v) = value.get(i)
                && let Value::Chars(s) = v.get_value()
                && *s == self.value
            {
                return Ok(());
            }
            return fail
                .context(ctx_desc("<pipe> | not exists"))
                .parse_next(&mut "");
        }
        for v in value.iter() {
            if v.get_name() == target
                && let Value::Chars(v) = v.get_value()
                && *v == self.value
            {
                return Ok(());
            }
        }
        fail.context(ctx_desc("<pipe> | not exists"))
            .parse_next(&mut "")
    }
}

impl DFPipeProcessor for PFFdExists {
    #[inline]
    fn process(&self, value: &mut Vec<DataField>, index: Option<&FieldIndex>) -> WResult<()> {
        let target = self.found.as_str();
        if let Some(ix) = index
            && let Some(i) = ix.get(target)
            && value.get(i).is_some()
        {
            return Ok(());
        }
        for v in value.iter() {
            if v.get_name() == target {
                return Ok(());
            }
        }
        fail.context(ctx_desc("json not exists sub item"))
            .parse_next(&mut "")
    }
}
impl DFPipeProcessor for PFCharsNotExists {
    #[inline]
    fn process(&self, value: &mut Vec<DataField>, index: Option<&FieldIndex>) -> WResult<()> {
        let target = self.target.as_str();
        let mut is_exists = false;
        if let Some(ix) = index {
            if let Some(i) = ix.get(target)
                && let Some(v) = value.get(i)
            {
                is_exists = true;
                if let Value::Chars(s) = v.get_value()
                    && *s != self.value
                {
                    return Ok(());
                }
            }
        } else {
            for v in value.iter() {
                if v.get_name() == target {
                    is_exists = true;
                    if let Value::Chars(vs) = v.get_value()
                        && *vs != self.value
                    {
                        return Ok(());
                    }
                }
            }
        }
        if !is_exists {
            return Ok(());
        }
        fail.context(ctx_desc("<pipe> | not exists"))
            .parse_next(&mut "")
    }
}

impl DFPipeProcessor for PFDigitExists {
    #[inline]
    fn process(&self, value: &mut Vec<DataField>, index: Option<&FieldIndex>) -> WResult<()> {
        let target = self.target.as_str();
        if let Some(ix) = index
            && let Some(i) = ix.get(target)
        {
            if let Some(v) = value.get(i)
                && let Value::Digit(d) = v.get_value()
                && *d == self.value
            {
                return Ok(());
            }
            return fail
                .context(ctx_desc("<pipe> | not exists"))
                .parse_next(&mut "");
        }
        for v in value.iter() {
            if v.get_name() == target
                && let Value::Digit(d) = v.get_value()
                && *d == self.value
            {
                return Ok(());
            }
        }
        fail.context(ctx_desc("<pipe> | not exists"))
            .parse_next(&mut "")
    }
}

impl DFPipeProcessor for PFDigitIn {
    #[inline]
    fn process(&self, value: &mut Vec<DataField>, index: Option<&FieldIndex>) -> WResult<()> {
        let target = self.target.as_str();
        if let Some(ix) = index
            && let Some(i) = ix.get(target)
        {
            if let Some(v) = value.get(i)
                && let Value::Digit(d) = v.get_value()
                && self.value.contains(d)
            {
                return Ok(());
            }
            return fail
                .context(ctx_desc("<pipe> | not in"))
                .parse_next(&mut "");
        }
        for v in value.iter() {
            if v.get_name() == target
                && let Value::Digit(d) = v.get_value()
                && self.value.contains(d)
            {
                return Ok(());
            }
        }
        fail.context(ctx_desc("<pipe> | not in"))
            .parse_next(&mut "")
    }
}

impl DFPipeProcessor for PFCharsIn {
    #[inline]
    fn process(&self, value: &mut Vec<DataField>, index: Option<&FieldIndex>) -> WResult<()> {
        let target = self.target.as_str();
        if let Some(ix) = index
            && let Some(i) = ix.get(target)
        {
            if let Some(v) = value.get(i)
                && let Value::Chars(s) = v.get_value()
                && self.value.contains(s)
            {
                return Ok(());
            }
            return fail
                .context(ctx_desc("<pipe> | not in"))
                .parse_next(&mut "");
        }
        for v in value.iter() {
            if v.get_name() == target
                && let Value::Chars(s) = v.get_value()
                && self.value.contains(s)
            {
                return Ok(());
            }
        }
        fail.context(ctx_desc("<pipe> | not in"))
            .parse_next(&mut "")
    }
}

impl DFPipeProcessor for PFIpAddrIn {
    #[inline]
    fn process(&self, value: &mut Vec<DataField>, index: Option<&FieldIndex>) -> WResult<()> {
        let target = self.target.as_str();
        if let Some(ix) = index
            && let Some(i) = ix.get(target)
        {
            if let Some(v) = value.get(i)
                && let Value::IpAddr(a) = v.get_value()
                && self.value.contains(a)
            {
                return Ok(());
            }
            return fail
                .context(ctx_desc("<pipe> | not in"))
                .parse_next(&mut "");
        }
        for v in value.iter() {
            if v.get_name() == target
                && let Value::IpAddr(a) = v.get_value()
                && self.value.contains(a)
            {
                return Ok(());
            }
        }
        fail.context(ctx_desc("<pipe> | not in"))
            .parse_next(&mut "")
    }
}
impl DFPipeProcessor for StubFun {
    #[inline]
    fn process(&self, _value: &mut Vec<DataField>, _index: Option<&FieldIndex>) -> WResult<()> {
        Ok(())
    }
}
impl DFPipeProcessor for WplFun {
    #[inline]
    fn process(&self, value: &mut Vec<DataField>, index: Option<&FieldIndex>) -> WResult<()> {
        match self {
            WplFun::CharsExists(f) => f.process(value, index),
            WplFun::CharsNotExists(f) => f.process(value, index),
            WplFun::CharsIn(f) => f.process(value, index),
            WplFun::DigitExists(f) => f.process(value, index),
            WplFun::DigitIn(f) => f.process(value, index),
            WplFun::IpAddrIn(f) => f.process(value, index),
            WplFun::Exists(f) => f.process(value, index),
            WplFun::StrMode(f) => f.process(value, index),
        }
    }
}

// ---------------- String Mode ----------------
use crate::ast::processor::PFStrMode;

impl PFStrMode {
    #[inline]
    fn is_decoded(&self) -> bool {
        matches!(self.mode.as_str(), "decoded" | "decode" | "dec")
    }
}

#[inline]
fn decode_json_escapes(raw: &str) -> Option<String> {
    // 通过 serde_json 反转义 JSON 字符串：重新包裹引号再解析
    let quoted = format!("\"{}\"", raw);
    serde_json::from_str::<String>(&quoted).ok()
}

#[inline]
fn transform_value_to_decoded(v: &mut Value) {
    match v {
        Value::Chars(s) => {
            // fast path: 没有反斜杠则无需反转义
            if !s.as_bytes().contains(&b'\\') {
                return;
            }
            if let Some(decoded) = decode_json_escapes(s) {
                *s = decoded;
            }
        }
        Value::Array(arr) => {
            for f in arr.iter_mut() {
                transform_value_to_decoded(f.get_value_mut());
            }
        }
        Value::Obj(obj) => {
            for (_k, f) in obj.iter_mut() {
                transform_value_to_decoded(f.get_value_mut());
            }
        }
        _ => {}
    }
}

impl DFPipeProcessor for PFStrMode {
    #[inline]
    fn process(&self, value: &mut Vec<DataField>, _index: Option<&FieldIndex>) -> WResult<()> {
        if !self.is_decoded() {
            return Ok(());
        }
        for f in value.iter_mut() {
            let v = f.get_value_mut();
            transform_value_to_decoded(v);
        }
        Ok(())
    }
}
