use std::net::IpAddr;

#[derive(Clone, Debug, PartialEq)]
pub struct ExistsChars(pub(crate) String);
#[derive(Clone, Debug, PartialEq)]
pub struct PFFdExists {
    pub(crate) found: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PFCharsExists {
    pub(crate) target: String,
    pub(crate) value: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PFCharsNotExists {
    pub(crate) target: String,
    pub(crate) value: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PFDigitExists {
    pub(crate) target: String,
    pub(crate) value: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PFDigitIn {
    pub(crate) target: String,
    pub(crate) value: Vec<i64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PFCharsIn {
    pub(crate) target: String,
    pub(crate) value: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PFIpAddrIn {
    pub(crate) target: String,
    pub(crate) value: Vec<IpAddr>,
}

impl PFFdExists {
    pub fn new<S: Into<String>>(found: S) -> Self {
        Self {
            found: found.into(),
        }
    }
}

#[derive(Clone, Default)]
#[allow(dead_code)]
pub struct StubFun {}

#[derive(Clone, Debug, PartialEq)]
pub struct PFStrMode {
    pub(crate) mode: String,
}
