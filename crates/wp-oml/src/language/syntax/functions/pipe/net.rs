use crate::language::prelude::*;

pub const PIPE_IP4_INT: &str = "to_ip4_int";

#[derive(Clone, Debug, Default)]
pub struct PipeIp4Int {}

impl Display for PipeIp4Int {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", PIPE_IP4_INT)
    }
}
