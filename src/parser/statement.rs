use nom::{
    IResult, 
    branch::{alt},
    combinator::{cut, opt},
    bytes::complete::tag,
};

use super::*;

pub fn statement(inp: &str) -> IResult<&str, Statement, Error> {
    alt((statement_if, statement_ret))(inp)
}

fn statement_if(inp: &str) -> IResult<&str, Statement, Error> {
    let (inp, _) = lexeme_ws(tag("if"))(inp)?;
    cut(|inp|{
        let (inp, g) = expression(inp)?;
        let (inp, then) = block(inp)?;

        let (inp, optelse) = opt(|inp| {
            let (inp, _) = lexeme_ws(tag("else"))(inp)?;
            let (inp, elsepart) = block(inp)?;
            Ok((inp, elsepart))
        })(inp)?;

        Ok((inp, Statement::If(g, then, optelse)))
    })(inp)
}

fn statement_ret(inp: &str) -> IResult<&str, Statement, Error> {
    let (inp, _) = lexeme_ws(tag("ret"))(inp)?;
    cut(|inp| {
        let (inp, expr) = expression(inp)?;
        let (inp, _) = lexeme(tag("."))(inp)?;

        Ok((inp, Statement::Ret(expr)))
    })(inp)
}