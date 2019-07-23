use nom::{
    IResult, 
    branch::{alt},
    combinator::{opt},
    bytes::complete::{take_while1, tag},
    multi, 
};

use std::str::FromStr;
use super::*;

pub fn pattern(inp: &str) -> IResult<&str, Pattern, Error> {
    alt((
        pattern_int_literal,

        pattern_variable,

        pattern_vector_literal,
        pattern_compound_literal,
    ))(inp)
}

fn pattern_int_literal(inp: &str) -> IResult<&str, Pattern, Error> {
    let (inp, digits) = lexeme(take_while1(|i| "0123456789".contains(i)))(inp)?;
    let ival = match i64::from_str(digits) {
        Ok(i) => i,
        Err(_) => { panic!("TODO: Implement") }
    };
    Ok((inp, Pattern::IntLiteral(ival)))
}

fn pattern_variable(inp: &str) -> IResult<&str, Pattern, Error> {
    let (inp, s) = var(inp)?;
    Ok((inp, Pattern::Variable(s)))
}

fn pattern_compound_literal(inp: &str) -> IResult<&str, Pattern, Error> {
    // TODO: Take a generalized string (quotes etc)
    let (inp, _) = tag(":")(inp)?;
    let (inp, head) = lexeme(identifier)(inp)?;
    let (inp, oargs) = opt(surrounded("(", ")", multi::separated_nonempty_list(lexeme(tag(",")), pattern)))(inp)?;
    let args = oargs.unwrap_or_else(|| vec![]);

    Ok((inp, Pattern::Compound(head, args)))
}

fn pattern_vector_literal(inp: &str) -> IResult<&str, Pattern, Error> {
    let (inp, args) = surrounded("v[", "]", multi::separated_list(lexeme(tag(",")), pattern))(inp)?;

    Ok((inp, Pattern::Vector(args)))
}