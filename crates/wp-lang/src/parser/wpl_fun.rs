use std::net::IpAddr;

use winnow::{
    Parser,
    ascii::{digit1, multispace0},
    combinator::alt,
};
use wp_parser::WResult;
use wp_parser::{
    atom::take_path,
    fun::{
        fun_trait::{Fun1Builder, Fun2Builder, ParseNext},
        parser::{call_fun_args1, call_fun_args2, take_arr},
    },
};

use crate::ast::{
    WplFun,
    processor::{
        ExistsChars, PFCharsExists, PFCharsIn, PFCharsNotExists, PFDigitExists, PFDigitIn,
        PFFdExists, PFIpAddrIn,
    },
};

use super::utils::take_key;

pub fn wpl_fun(input: &mut &str) -> WResult<WplFun> {
    multispace0.parse_next(input)?;
    let fun = alt((
        call_fun_args2::<PFCharsExists>.map(WplFun::CharsExists),
        call_fun_args2::<PFCharsNotExists>.map(WplFun::CharsNotExists),
        call_fun_args2::<PFCharsIn>.map(WplFun::CharsIn),
        call_fun_args2::<PFDigitExists>.map(WplFun::DigitExists),
        call_fun_args2::<PFDigitIn>.map(WplFun::DigitIn),
        call_fun_args2::<PFIpAddrIn>.map(WplFun::IpAddrIn),
        call_fun_args1::<PFFdExists>.map(WplFun::Exists),
        call_fun_args1::<PFStrMode>.map(WplFun::StrMode),
    ))
    .parse_next(input)?;
    Ok(fun)
}

impl Fun2Builder for PFDigitExists {
    type ARG1 = String;
    type ARG2 = i64;

    fn args1(data: &mut &str) -> WResult<Self::ARG1> {
        multispace0.parse_next(data)?;
        let val = take_key.parse_next(data)?;
        Ok(val.to_string())
    }
    fn args2(data: &mut &str) -> WResult<Self::ARG2> {
        multispace0.parse_next(data)?;
        let val = digit1.parse_next(data)?;
        Ok(val.parse::<i64>().unwrap_or(0))
    }

    fn fun_name() -> &'static str {
        "exists_digit"
    }

    fn build(args: (Self::ARG1, Self::ARG2)) -> Self {
        Self {
            target: args.0,
            value: args.1,
        }
    }
}
impl Fun2Builder for PFCharsExists {
    type ARG1 = String;
    type ARG2 = String;

    fn args1(data: &mut &str) -> WResult<Self::ARG1> {
        multispace0.parse_next(data)?;
        let val = take_key.parse_next(data)?;
        Ok(val.to_string())
    }
    fn args2(data: &mut &str) -> WResult<Self::ARG2> {
        multispace0.parse_next(data)?;
        let val = take_path.parse_next(data)?;
        Ok(val.to_string())
    }

    fn fun_name() -> &'static str {
        "exists_chars"
    }
    fn build(args: (Self::ARG1, Self::ARG2)) -> Self {
        Self {
            target: args.0,
            value: args.1,
        }
    }
}
impl Fun2Builder for PFCharsNotExists {
    type ARG1 = String;
    type ARG2 = String;

    fn args1(data: &mut &str) -> WResult<Self::ARG1> {
        multispace0.parse_next(data)?;
        let val = take_key.parse_next(data)?;
        Ok(val.to_string())
    }
    fn args2(data: &mut &str) -> WResult<Self::ARG2> {
        multispace0.parse_next(data)?;
        let val = take_path.parse_next(data)?;
        Ok(val.to_string())
    }

    fn fun_name() -> &'static str {
        "chars_not_exists"
    }
    fn build(args: (Self::ARG1, Self::ARG2)) -> Self {
        Self {
            target: args.0,
            value: args.1,
        }
    }
}
impl ParseNext<ExistsChars> for ExistsChars {
    fn parse_next(input: &mut &str) -> WResult<ExistsChars> {
        let val = take_path.parse_next(input)?;
        Ok(ExistsChars(val.to_string()))
    }
}
impl Fun2Builder for PFCharsIn {
    type ARG1 = String;
    type ARG2 = Vec<ExistsChars>;
    fn args1(data: &mut &str) -> WResult<Self::ARG1> {
        multispace0.parse_next(data)?;
        let val = take_key.parse_next(data)?;
        Ok(val.to_string())
    }

    fn args2(data: &mut &str) -> WResult<Self::ARG2> {
        take_arr::<ExistsChars>(data)
    }

    fn fun_name() -> &'static str {
        "exists_chars_in"
    }

    fn build(args: (Self::ARG1, Self::ARG2)) -> Self {
        let value: Vec<String> = args.1.iter().map(|i| i.0.clone()).collect();
        Self {
            target: args.0,
            value,
        }
    }
}

impl Fun2Builder for PFDigitIn {
    type ARG1 = String;
    type ARG2 = Vec<i64>;

    fn args2(data: &mut &str) -> WResult<Self::ARG2> {
        take_arr::<i64>(data)
    }
    fn args1(data: &mut &str) -> WResult<Self::ARG1> {
        multispace0.parse_next(data)?;
        let val = take_key.parse_next(data)?;
        Ok(val.to_string())
    }

    fn fun_name() -> &'static str {
        "exists_digit_in"
    }
    fn build(args: (Self::ARG1, Self::ARG2)) -> Self {
        Self {
            target: args.0,
            value: args.1,
        }
    }
}
impl Fun1Builder for PFFdExists {
    type ARG1 = String;

    fn args1(data: &mut &str) -> WResult<Self::ARG1> {
        multispace0.parse_next(data)?;
        let val = take_key.parse_next(data)?;
        Ok(val.to_string())
    }

    fn fun_name() -> &'static str {
        "exists"
    }

    fn build(args: Self::ARG1) -> Self {
        Self { found: args }
    }
}

impl Fun2Builder for PFIpAddrIn {
    type ARG1 = String;
    type ARG2 = Vec<IpAddr>;

    fn args2(data: &mut &str) -> WResult<Self::ARG2> {
        take_arr::<IpAddr>(data)
    }
    fn args1(data: &mut &str) -> WResult<Self::ARG1> {
        multispace0.parse_next(data)?;
        let val = take_key.parse_next(data)?;
        Ok(val.to_string())
    }

    fn fun_name() -> &'static str {
        "exists_ip_in"
    }
    fn build(args: (Self::ARG1, Self::ARG2)) -> Self {
        Self {
            target: args.0,
            value: args.1,
        }
    }
}

// ---------------- String Mode ----------------
use crate::ast::processor::PFStrMode;

impl Fun1Builder for PFStrMode {
    type ARG1 = String;

    fn args1(data: &mut &str) -> WResult<Self::ARG1> {
        use winnow::token::take_till;
        multispace0.parse_next(data)?;
        let val = take_till(1.., |c: char| c == ')' || c == ',').parse_next(data)?;
        Ok(val.trim().to_string())
    }

    fn fun_name() -> &'static str {
        "str_mode"
    }

    fn build(args: Self::ARG1) -> Self {
        PFStrMode { mode: args }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr};

    use orion_error::TestAssert;

    use crate::ast::processor::PFFdExists;

    use super::*;

    #[test]
    fn test_parse_fun() {
        let fun = wpl_fun.parse(r#"exists(src)"#).assert();
        assert_eq!(
            fun,
            WplFun::Exists(PFFdExists {
                found: "src".to_string()
            })
        );

        let fun = wpl_fun.parse(r#"exists_digit_in(src, [1,2,3])"#).assert();
        assert_eq!(
            fun,
            WplFun::DigitIn(PFDigitIn {
                target: "src".to_string(),
                value: vec![1, 2, 3]
            })
        );

        let fun = wpl_fun
            .parse(r#"exists_ip_in(src, [127.0.0.1, 127.0.0.2])"#)
            .assert();
        assert_eq!(
            fun,
            WplFun::IpAddrIn(PFIpAddrIn {
                target: "src".to_string(),
                value: vec![
                    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2))
                ]
            })
        );

        // IPv6 裸字面量与混合示例
        let fun = wpl_fun
            .parse(r#"exists_ip_in(src, [::1, 2001:db8::1])"#)
            .assert();
        assert_eq!(
            fun,
            WplFun::IpAddrIn(PFIpAddrIn {
                target: "src".to_string(),
                value: vec![
                    IpAddr::V6(Ipv6Addr::LOCALHOST),
                    IpAddr::V6("2001:db8::1".parse().unwrap()),
                ]
            })
        );
    }
}
