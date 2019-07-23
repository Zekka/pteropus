use nom::{
    IResult, 
    branch::{alt},
    combinator::{cut, opt},
    bytes::complete::{take_while1, tag},
    multi,
};

use std::str::FromStr;
use super::*;

pub fn condition(inp: &str) -> IResult<&str, Condition, Error> {
    alt((
        condition_let,
        |inp| expression(inp).map(|(i, o)| (i, Condition::Bare(o))),
    ))(inp)
}

fn condition_let(inp: &str) -> IResult<&str, Condition, Error> {
    let (inp, _) = lexeme_ws(tag("let"))(inp)?;
    let (inp, pat) = pattern(inp)?;
    let (inp, _) = lexeme(tag("="))(inp)?;
    let (inp, expr) = expression(inp)?;
    Ok((inp, Condition::Let(pat, expr)))
}

pub fn expression(inp: &str) -> IResult<&str, Expression, Error> {
    let (mut inp, mut lhs) = expression_leaf(inp)?;
    loop {
        // if it consumes nothing, quit
        match opt(expression_binop)(inp)? {
            (i, None) => {
                return Ok((i, lhs));
            }
            (i, Some(opt)) => {
                let (i_, rhs) = expression(i)?;
                inp = i_;
                lhs = lhs.integrate(opt, rhs);
            }
        }
    };
}

fn expression_binop(inp: &str) -> IResult<&str, BinOp, Error> {
    lexeme(alt((
        |inp| { let (inp, _) = tag("*")(inp)?; Ok((inp, BinOp::Multiply)) },
        |inp| { let (inp, _) = tag("/")(inp)?; Ok((inp, BinOp::Divide)) },
        |inp| { let (inp, _) = tag("+")(inp)?; Ok((inp, BinOp::Add)) },
        |inp| { let (inp, _) = tag("-")(inp)?; Ok((inp, BinOp::Subtract)) },

        |inp| { let (inp, _) = tag("<=")(inp)?; Ok((inp, BinOp::Le)) },
        |inp| { let (inp, _) = tag(">=")(inp)?; Ok((inp, BinOp::Ge)) },
        |inp| { let (inp, _) = tag("<")(inp)?; Ok((inp, BinOp::Lt)) },
        |inp| { let (inp, _) = tag(">")(inp)?; Ok((inp, BinOp::Gt)) },
        |inp| { let (inp, _) = tag("==")(inp)?; Ok((inp, BinOp::Eq)) },
        |inp| { let (inp, _) = tag("!=")(inp)?; Ok((inp, BinOp::Ne)) },
    )))(inp)
}

fn expression_leaf(inp: &str) -> IResult<&str, Expression, Error> {
    alt((
        expression_int_literal,

        expression_variable,
        expression_call,

        expression_vector_literal,
        expression_set_literal,
        expression_compound_literal,

        expression_implicit_call,
    ))(inp)
}

// TODO: Term literal. Don't have string literals, only term literals

fn expression_int_literal(inp: &str) -> IResult<&str, Expression, Error> {
    let (inp, digits) = lexeme(take_while1(|i| "0123456789".contains(i)))(inp)?;
    let ival = match i64::from_str(digits) {
        Ok(i) => i,
        Err(_) => { panic!("TODO: Implement") }
    };
    Ok((inp, Expression::IntLiteral(ival)))
}

fn expression_variable(inp: &str) -> IResult<&str, Expression, Error> {
    let (inp, s) = var(inp)?;
    Ok((inp, Expression::Variable(s)))
}

fn expression_call(inp: &str) -> IResult<&str, Expression, Error> {
    let (inp, _) = lexeme_ws(tag("call"))(inp)?;
    cut(|inp| {
        let (inp, term) = expression_leaf(inp)?;
        Ok((inp, Expression::Call(box term)))
    })(inp)
}

fn expression_compound_literal(inp: &str) -> IResult<&str, Expression, Error> {
    let (inp, _) = tag(":")(inp)?;
    cut(|inp| {
        let (inp, head) = lexeme(identifier)(inp)?;
        let (inp, oargs) = opt(surrounded("(", ")", multi::separated_nonempty_list(lexeme(tag(",")), expression)))(inp)?;
        let args = oargs.unwrap_or_else(|| vec![]);

        Ok((inp, Expression::Compound(head, args)))
    })(inp)
}

fn expression_vector_literal(inp: &str) -> IResult<&str, Expression, Error> {
    let (inp, args) = surrounded("v[", "]", multi::separated_list(lexeme(tag(",")), expression))(inp)?;

    Ok((inp, Expression::Vector(args)))
}

fn expression_set_literal(inp: &str) -> IResult<&str, Expression, Error> {
    let (inp, args) = surrounded("s[", "]", multi::separated_list(lexeme(tag(",")), expression))(inp)?;

    Ok((inp, Expression::Set(args)))
}

pub fn expression_implicit_call(inp: &str) -> IResult<&str, Expression, Error> {
    // copypasted expression_compound_literal body
    let (inp, head) = lexeme(identifier)(inp)?;
    cut(move |inp| {
        let (inp, oargs) = opt(surrounded("(", ")", multi::separated_nonempty_list(lexeme(tag(",")), expression)))(inp)?;
        let args = oargs.unwrap_or_else(|| vec![]);

        Ok((inp, Expression::Call(box Expression::Compound(head.clone(), args))))
    })(inp)
}

