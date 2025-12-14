pub const FUN_TIME_NOW: &str = "Time::now";
pub const FUN_TIME_NOW_DATE: &str = "Time::now_date";
pub const FUN_TIME_NOW_TIME: &str = "Time::now_time";
pub const FUN_TIME_NOW_HOUR: &str = "Time::now_hour";

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct FunNow {}
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct FunNowDate {}
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct FunNowTime {}
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct FunNowHour {}
