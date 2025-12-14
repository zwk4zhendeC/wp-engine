use std::fmt::Write;

use crate::ast::group::{GroupSeq, WplGroupType};
use crate::ast::{WplField, WplSep};
//use crate::engine::arsenal::ParseArsenals;
use crate::eval::runtime::field::FieldEvalUnit;
use crate::eval::runtime::vm_unit::WplEvaluator;
use crate::eval::value::parse_def::FieldParser;
use crate::eval::vof;
use crate::generator::GenChannel;
use crate::generator::ParserValue;
use orion_error::TestAssert;
use wp_data_fmt::{DataFormat, Raw};
use wp_model_core::model::DataField;
use wp_model_core::model::DataType;
use wp_parser::WResult as ModalResult;

pub struct ParserTestEnv {
    pub gch: GenChannel,
    //pub asr: ParseArsenals,
}

impl ParserTestEnv {
    pub fn new() -> Self {
        //let asr = ParseArsenals::default();

        let gch = GenChannel::new();
        ParserTestEnv { gch }
    }
}

impl Default for ParserTestEnv {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ParserTUnit {
    //lang: T,
    fpu: FieldEvalUnit,
    env: ParserTestEnv,
    ups_sep: WplSep,
}
impl ParserTUnit {
    pub fn new<T: FieldParser + Send + Sync + 'static>(parser: T, conf: WplField) -> Self {
        let env = ParserTestEnv::new();
        let fpu = FieldEvalUnit::for_test(parser, conf);
        let ups_sep = WplSep::default();
        //let fpu = LangPipe::assemble_fpu(&conf).expect("assemble fpu fail");
        Self { fpu, env, ups_sep }
    }
    pub fn from_auto(conf: WplField) -> Self {
        let env = ParserTestEnv::new();
        let ups_sep = WplSep::default();
        //let fpu = FieldProcUnit::for_test(lang,conf);
        let fpu = WplEvaluator::assemble_fpu(0, &conf, WplGroupType::Seq(GroupSeq))
            .expect("assemble fpu fail");
        Self { fpu, env, ups_sep }
    }
}

#[allow(dead_code)]
pub struct ParserTUnit2 {}

impl ParserTUnit2 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let _env = ParserTestEnv::new();
        ParserTUnit2 {}
    }
}

impl Default for ParserTUnit2 {
    fn default() -> Self {
        Self::new()
    }
}

pub fn verify_parse_v_suc_end<T, V>(data: &mut &str) -> V
where
    T: ParserValue<V>,
{
    match T::parse_value(data) {
        Ok(field) => {
            assert!(data.is_empty());
            field
        }
        Err(e) => {
            panic!("parse error: {}", e);
        }
    }
}

impl ParserTUnit {
    pub fn verify_parse_suc(self, data: &mut &str) -> ModalResult<Vec<DataField>> {
        //self.fpu.exec(data, Some(self.fpu.conf().safe_name()))
        let mut out = Vec::new();
        self.fpu.parse(&self.ups_sep, data, None, &mut out)?;
        Ok(out)
    }

    pub fn verify_parse_suc_meta(&mut self, data: &mut &str, meta: DataType) -> Vec<DataField> {
        let mut out = Vec::new();
        self.fpu
            .parse(
                &self.ups_sep,
                data,
                Some(self.fpu.conf().safe_name()),
                &mut out,
            )
            .expect("parse error");
        println!("{}", out[0]);
        assert_eq!(*out[0].get_meta(), meta);
        out
    }

    pub fn verify_parse_suc_end(&mut self, data: &mut &str) -> ModalResult<Vec<DataField>> {
        let mut field = Vec::new();
        self.fpu
            .parse(
                &self.ups_sep,
                data,
                Some(self.fpu.conf().safe_name()),
                &mut field,
            )
            .expect("parse error");
        assert_eq!(*field[0].get_meta(), self.fpu.conf().meta_type().clone());
        assert_eq!(&*data, &"");
        Ok(field)
    }
    pub fn verify_parse_suc_end_meta(&mut self, data: &mut &str, meta: DataType) -> Vec<DataField> {
        let mut field = Vec::new();
        self.fpu
            .parse(
                &self.ups_sep,
                data,
                Some(self.fpu.conf().safe_name()),
                &mut field,
            )
            .expect("parse error");
        assert_eq!(*field[0].get_meta(), meta);
        assert_eq!(*data, "");
        field
    }

    pub fn verify_parse_fail(&mut self, data: &mut &str) {
        let mut field = Vec::new();
        assert!(
            self.fpu
                .parse(
                    &self.ups_sep,
                    data,
                    Some(self.fpu.conf().safe_name()),
                    &mut field
                )
                .is_err()
        )
    }
    pub fn verify_gen_parse_suc(&mut self) {
        verify_gen_parse(&mut self.env, &self.fpu, self.fpu.conf());
    }
}

pub fn verify_gen_parse(env: &mut ParserTestEnv, fpu: &FieldEvalUnit, conf: &WplField) {
    let cur_sep = WplSep::default();
    let mut buffer = String::new();
    let fmt_field = fpu.generate(&mut env.gch, &cur_sep, None).assert();
    let rawfmt = Raw::new();
    buffer
        .write_fmt(format_args!(
            "{}{}{}{}",
            vof(fmt_field.field_fmt.scope_beg, ""),
            rawfmt.format_field(&fmt_field.data_field),
            cur_sep.sep_str(),
            vof(fmt_field.field_fmt.scope_end, "")
        ))
        .expect("panic message");
    println!("gen data:{}", buffer);
    let mut data = buffer.as_str();
    let mut field = Vec::new();
    match fpu.parse(
        &cur_sep,
        &mut data,
        Some(fpu.conf().safe_name()),
        &mut field,
    ) {
        Ok(_) => {
            assert_eq!(data, "");
            assert_eq!(field[0].get_meta(), conf.meta_type());
        }
        Err(e) => {
            panic!("parse error: {}", e);
        }
    }
}
