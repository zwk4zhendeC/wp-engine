use crate::language::prelude::*;
use wp_parser::fun::fun_trait::Fun2Builder;
pub const PIPE_TIMESTAMP: &str = "to_timestamp";
#[derive(Clone, Debug, Default)]
pub struct PipeTimeStamp {}

pub const PIPE_TIMESTAMP_MS: &str = "to_timestamp_ms";
#[derive(Clone, Debug, Default)]
pub struct PipeTimeStampMS {}

#[derive(Clone, Debug, Default, Display)]
#[display(style = "snake_case")]
pub enum TimeStampUnit {
    MS,
    US,
    #[default]
    SS,
}
pub const PIPE_TIMESTAMP_US: &str = "to_timestamp_us";
#[derive(Clone, Debug, Default)]
pub struct PipeTimeStampUS {}
pub const PIPE_TIMESTAMP_ZONE: &str = "to_timestamp_zone";
#[derive(Clone, Debug, Default, Builder)]
pub struct PipeTimeStampZone {
    pub(crate) unit: TimeStampUnit,
    pub(crate) zone: i32,
}
impl Display for PipeTimeStampZone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({},{})", Self::fun_name(), self.zone, self.unit)
    }
}
