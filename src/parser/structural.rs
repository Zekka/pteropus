use nom::{
    IResult, 
    branch::alt,
    combinator::cut,
    bytes::complete::tag,
    multi,
};

use super::*;

pub fn module(inp: &str) -> IResult<&str, Module, Error> {
    let (inp, res) = multi::many0(procedure)(inp)?;
    Ok((inp, Module { procedures: res }))
}

pub fn procedure(inp: &str) -> IResult<&str, crate::ast::Procedure, Error> {
    let (inp, _) = lexeme_ws(tag("fn"))(inp)?;
    return cut(|inp| {
        let (inp, identifier) = identifier(inp)?;
        let (inp, args) = alt((
            surrounded("(", ")", multi::separated_nonempty_list(lexeme(tag(",")), pattern)),
            |inp| Ok((inp, vec![])),
        ))(inp)?;
        let (inp, body) = block(inp)?;
        Ok((inp, Procedure {
            name: identifier,
            args: args,
            body: body,
        }))
    })(inp)
}

pub fn block(inp: &str) -> IResult<&str, Block, Error> {
    let (inp, sts) = surrounded("{", "}", multi::many0(statement))(inp)?;
    Ok((inp, Block(sts)))
}