use nom::{
    IResult, 
    bytes::complete::{tag},
    character::complete::{one_of},
    multi,
};

use super::*;

pub fn var(inp: &str) -> IResult<&str, String, Error> {
    // the @ is not a lexeme: can't be followed by whitespace
    let (inp, _) = tag("@")(inp)?;
    identifier(inp)
}

pub fn identifier(inp: &str) -> IResult<&str, String, Error> {
    let (inp, first) = one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_")(inp)?;
    let (inp, remaining) = lexeme(multi::many0(one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_")))(inp)?;

    let mut result = String::with_capacity(1 + remaining.len());
    result.push(first);
    for c in remaining { result.push(c); }
    Ok((inp, result))
}