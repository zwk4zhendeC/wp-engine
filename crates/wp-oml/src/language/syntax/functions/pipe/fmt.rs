use crate::language::prelude::*;
pub const PIPE_TO_JSON: &str = "to_json";
#[derive(Default, Builder, Debug, Clone, Getters, Serialize, Deserialize)]
pub struct PipeToJson {}
pub const PIPE_JSON_ESCAPE_EN: &str = "json_escape_en";
#[derive(Clone, Debug, Default)]
pub struct PipeJsonEscapeEN {}

pub const PIPE_JSON_ESCAPE_DE: &str = "json_escape_de";
#[derive(Clone, Debug, Default)]
pub struct PipeJsonEscapeDE {}
