use crate::language::{
    BuiltinFunction, FUN_TIME_NOW, FUN_TIME_NOW_DATE, FUN_TIME_NOW_TIME, FunNowTime, FunOperation,
};
use crate::language::{FUN_TIME_NOW_HOUR, FunNow, FunNowDate, FunNowHour, PreciseEvaluator};
use winnow::ascii::multispace0;
use winnow::combinator::alt;
use wp_parser::Parser;
use wp_parser::WResult;
use wp_parser::utils::get_scope;

pub fn oml_gw_fun(data: &mut &str) -> WResult<PreciseEvaluator> {
    let fun = oml_fun_item.parse_next(data)?;
    Ok(PreciseEvaluator::Fun(FunOperation::new(fun)))
}

pub fn oml_fun_item(data: &mut &str) -> WResult<BuiltinFunction> {
    multispace0.parse_next(data)?;
    let fun = alt((
        FUN_TIME_NOW_DATE.map(|_| BuiltinFunction::NowDate(FunNowDate::default())),
        FUN_TIME_NOW_HOUR.map(|_| BuiltinFunction::NowHour(FunNowHour::default())),
        FUN_TIME_NOW_TIME.map(|_| BuiltinFunction::NowTime(FunNowTime::default())),
        FUN_TIME_NOW.map(|_| BuiltinFunction::Now(FunNow::default())),
    ))
    .parse_next(data)?;
    let _ = get_scope(data, '(', ')');
    Ok(fun)
}

#[cfg(test)]
mod tests {
    use crate::parser::fun_prm::oml_gw_fun;
    use crate::parser::utils::for_test::assert_oml_parse;
    use wp_parser::WResult as ModalResult;

    #[test]
    fn test_oml_crate_lib() -> ModalResult<()> {
        let mut code = r#" Time::now()
     "#;
        assert_oml_parse(&mut code, oml_gw_fun);

        let mut code = r#" Time::now_hour()
     "#;
        assert_oml_parse(&mut code, oml_gw_fun);

        let mut code = r#" Time::now_date()
     "#;
        assert_oml_parse(&mut code, oml_gw_fun);

        let mut code = r#" Time::now_time()
     "#;
        assert_oml_parse(&mut code, oml_gw_fun);

        Ok(())
    }
}
