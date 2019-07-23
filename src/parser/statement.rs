use nom::{
    IResult, 
    branch::{alt},
    combinator::{cut, opt},
    bytes::complete::tag,
};

use super::*;

pub fn statement(inp: &str) -> IResult<&str, Statement, Error> {
    alt((statement_let, statement_now, statement_eval, statement_if, statement_ret, statement_call))(inp)
}

fn statement_now(inp: &str) -> IResult<&str, Statement, Error> {
    let (inp, _) = lexeme_ws(tag("now"))(inp)?;
    cut(|inp| {
        let (inp, pat) = var(inp)?;
        let (inp, _) = lexeme(tag("="))(inp)?;
        let (inp, expr) = expression(inp)?;
        let (inp, _) = lexeme(tag("."))(inp)?;

        Ok((inp, Statement::Assign(pat, expr)))
    })(inp)
}

fn statement_let(inp: &str) -> IResult<&str, Statement, Error> {
    let (inp, _) = lexeme_ws(tag("let"))(inp)?;
    cut(|inp| {
        let (inp, pat) = pattern(inp)?;
        let (inp, _) = lexeme(tag("="))(inp)?;
        let (inp, expr) = expression(inp)?;
        let (inp, _) = lexeme(tag("."))(inp)?;

        Ok((inp, Statement::Destructure(pat, expr)))
    })(inp)
}

fn statement_eval(inp: &str) -> IResult<&str, Statement, Error> {
    let (inp, _) = lexeme_ws(tag("eval"))(inp)?;
    cut(|inp| {
        let (inp, expr) = expression(inp)?;
        let (inp, _) = lexeme(tag("."))(inp)?;

        Ok((inp, Statement::Eval(expr)))
    })(inp)
}

fn statement_if(inp: &str) -> IResult<&str, Statement, Error> {
    let (inp, _) = lexeme_ws(tag("if"))(inp)?;
    cut(|inp|{
        let (inp, g) = condition(inp)?;
        let (inp, then) = block(inp)?;

        let (inp, optelse) = opt(|inp| {
            let (inp, _) = lexeme_ws(tag("else"))(inp)?;
            let (inp, elsepart) = block_or_if(inp)?;
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

fn statement_call(inp: &str) -> IResult<&str, Statement, Error> {
    let (inp, expr) = expression_implicit_call(inp)?;
    let (inp, _) = lexeme(tag("."))(inp)?;
    Ok((inp, Statement::Eval(expr)))
}

fn block_or_if(inp: &str) -> IResult<&str, Block, Error> {
    alt((
        |inp| statement_if(inp).map(|(i, o)| (i, Block(vec![o]))),
        block,
    ))(inp)
}