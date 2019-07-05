use nom::{
    IResult, 
    branch::{alt},
    bytes::complete::{take_while1, take_till, tag},
    character::complete::anychar,
    multi, 
};

use super::*;

pub fn some_whitespace(inp: &str) -> IResult<&str, (), Error> {
    if inp == "" { return Ok((inp, ())); }
    let (inp, _) = multi::many1_count(alt((actual_whitespace, comment)))(inp)?;
    Ok((inp, ()))
}

pub fn any_whitespace(inp: &str) -> IResult<&str, (), Error> {
    let (inp, _) = multi::many0_count(alt((actual_whitespace, comment)))(inp)?;
    Ok((inp, ()))
}

fn actual_whitespace(inp: &str) -> IResult<&str, (), Error> {
    let (inp, _) = take_while1(|x| "\n\r \t".contains(x))(inp)?;
    Ok((inp, ()))
}

fn comment(inp: &str) -> IResult<&str, (), Error> {
    alt((line_comment, block_comment))(inp)
}

fn line_comment(inp: &str) -> IResult<&str, (), Error> {
    let (inp, _) = tag("//")(inp)?;
    let (inp, _) = take_till(|x| x == '\n')(inp)?;
    Ok((inp, ()))
}

fn block_comment(inp: &str) -> IResult<&str, (), Error> {
    let (inp, _) = tag("/*")(inp)?;
    let (inp, _) = multi::many_till(anychar, tag("*/"))(inp)?;
    Ok((inp, ()))
}
