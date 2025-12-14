use strum_macros::EnumString;

use crate::language::prelude::*;
pub const PIPE_BASE64_EN: &str = "base64_en";
#[derive(Default, Builder, Debug, Clone, Getters, Serialize, Deserialize)]
pub struct PipeBase64Encode {}

pub const PIPE_BASE64_DE: &str = "base64_de";
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PipeBase64Decode {
    pub encode: EncodeType,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize, EnumString, strum_macros::Display)]
pub enum EncodeType {
    #[default]
    Utf8,
    Utf16le,
    Utf16be,
    Windows949,
    EucJp,
    Windows31j,
    Iso2022Jp,
    Gbk,
    Gb18030,
    HZ,
    Big52003,
    MacCyrillic,
    Windows874,
    Windows1250,
    Windows1251,
    Windows1252,
    Windows1253,
    Windows1254,
    Windows1255,
    Windows1256,
    Windows1257,
    Windows1258,
    Ascii,
    Ibm866,
    Iso88591,
    Iso88592,
    Iso88593,
    Iso88594,
    Iso88595,
    Iso88596,
    Iso88597,
    Iso88598,
    Iso885910,
    Iso885913,
    Iso885914,
    Iso885915,
    Iso885916,
    Koi8R,
    Koi8U,
    MacRoman,
    Imap,
}

impl Display for PipeBase64Decode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", PIPE_BASE64_DE, self.encode)
    }
}
