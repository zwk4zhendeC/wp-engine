use super::function::{
    FCharsHas, FCharsIn, FCharsNotHas, FDigitHas, FDigitIn, FIpAddrIn, FdHas, JsonUnescape,
    SelectLast, TakeField,
};
use crate::ast::{group::WplGroup, processor::Base64Decode};

#[derive(Debug, Clone, PartialEq)]
pub enum WplFun {
    SelectTake(TakeField),
    SelectLast(SelectLast),
    FCharsExists(FCharsHas),
    FCharsNotExists(FCharsNotHas),
    FCharsIn(FCharsIn),
    FDigitExists(FDigitHas),
    FDigitIn(FDigitIn),
    FIpAddrIn(FIpAddrIn),
    FExists(FdHas),
    TransJsonUnescape(JsonUnescape),
    TransBase64Decode(Base64Decode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum WplPipe {
    Fun(WplFun),
    Group(WplGroup),
}
