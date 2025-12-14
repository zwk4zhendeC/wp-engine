use crate::core::prelude::*;
use crate::language::{
    PipeHtmlEscapeDecode, PipeHtmlEscapeEncode, PipeJsonEscapeDE, PipeJsonEscapeEN,
    PipeStrEscapeEN, PipeToJson, PipeToString,
};

use wp_data_fmt::{DataFormat, Json};
use wp_model_core::model::{DataField, DataType, Value};

impl ValueProcessor for PipeStrEscapeEN {
    fn value_cacu(&self, in_val: DataField) -> DataField {
        match in_val.get_value() {
            Value::Chars(x) => {
                let html = x.chars().flat_map(|c| c.escape_default()).collect();
                DataField::from_chars(in_val.get_name().to_string(), html)
            }
            _ => in_val,
        }
    }
}
impl ValueProcessor for PipeHtmlEscapeEncode {
    fn value_cacu(&self, in_val: DataField) -> DataField {
        match in_val.get_value() {
            Value::Chars(x) => {
                let mut html = String::new();
                html_escape::encode_safe_to_string(x, &mut html);
                //let html = html.replace("/&yen;/g", '￥');
                DataField::from_chars(in_val.get_name().to_string(), html)
            }
            _ => in_val,
        }
    }
}
impl ValueProcessor for PipeJsonEscapeEN {
    fn value_cacu(&self, in_val: DataField) -> DataField {
        match in_val.get_value() {
            Value::Chars(x) => {
                let json = escape8259::escape(x);
                DataField::from_chars(in_val.get_name().to_string(), json)
            }
            _ => in_val,
        }
    }
}
impl ValueProcessor for PipeJsonEscapeDE {
    fn value_cacu(&self, in_val: DataField) -> DataField {
        match in_val.get_value() {
            Value::Chars(x) => {
                if let Ok(json) = escape8259::unescape(x) {
                    DataField::from_chars(in_val.get_name().to_string(), json)
                } else {
                    in_val
                    //TDOEnum::Chars(x)
                }
            }
            _ => in_val,
        }
    }
}

impl ValueProcessor for PipeHtmlEscapeDecode {
    fn value_cacu(&self, in_val: DataField) -> DataField {
        match in_val.get_value() {
            Value::Chars(x) => {
                let mut html = String::new();
                html_escape::decode_html_entities_to_string(x, &mut html);
                DataField::from_chars(in_val.get_name().to_string(), html)
            }
            _ => in_val,
        }
    }
}
impl ValueProcessor for PipeToString {
    fn value_cacu(&self, in_val: DataField) -> DataField {
        match in_val.get_value() {
            Value::Chars(_) => in_val,
            Value::IpAddr(ip) => {
                DataField::from_chars(in_val.get_name().to_string(), ip.to_string())
            }
            _ => unimplemented!(),
        }
    }
}
impl ValueProcessor for PipeToJson {
    fn value_cacu(&self, in_val: DataField) -> DataField {
        let meta = DataType::Json;
        let json_fmt = Json;
        let json_str = json_fmt.fmt_value(in_val.get_value()).to_string();
        DataField::new(meta, in_val.clone_name(), json_str)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::DataTransformer;
    use crate::parser::oml_parse;
    use orion_error::TestAssert;
    use wp_data_model::cache::FieldQueryCache;
    use wp_model_core::model::{DataField, DataRecord};

    #[test]
    fn test_html_escape() {
        let cache = &mut FieldQueryCache::default();
        let data = vec![DataField::from_chars("A1", "<html>")];
        let src = DataRecord { items: data };

        let mut conf = r#"
        name : test
        ---
        X : chars =  pipe take(A1) | html_escape_en | html_escape_de;
         "#;
        let model = oml_parse(&mut conf).assert();

        let target = model.transform(src, cache);

        let expect = DataField::from_chars("X".to_string(), "<html>".to_string());
        assert_eq!(target.field("X"), Some(&expect));
    }

    #[test]
    fn test_str_escape() {
        let cache = &mut FieldQueryCache::default();
        let data = vec![DataField::from_chars("A1", "html\"1_")];
        let src = DataRecord { items: data };

        let mut conf = r#"
        name : test
        ---
        X : chars =  pipe take(A1) | str_escape_en  ;
         "#;
        let model = oml_parse(&mut conf).assert();

        let target = model.transform(src, cache);

        let expect = DataField::from_chars("X".to_string(), r#"html\"1_"#.to_string());
        assert_eq!(target.field("X"), Some(&expect));
    }

    #[test]
    fn test_json_escape() {
        let cache = &mut FieldQueryCache::default();
        let data = vec![DataField::from_chars("A1", "This is a crab: 🦀")];
        let src = DataRecord { items: data };

        let mut conf = r#"
        name : test
        ---
        X : chars =  pipe take(A1) | json_escape_en  | json_escape_de ;
         "#;
        let model = oml_parse(&mut conf).assert();

        let target = model.transform(src, cache);

        let expect = DataField::from_chars("X".to_string(), "This is a crab: 🦀".to_string());
        assert_eq!(target.field("X"), Some(&expect));
    }
}
