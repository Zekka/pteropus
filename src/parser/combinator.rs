use nom::{
    IResult, 
    combinator::{cut},
    bytes::complete::{tag},
};

use super::*;

pub fn lexeme_ws<'a, O>(f: impl Fn(&'a str) -> IResult<&'a str, O, Error>) -> impl Fn(&'a str) -> IResult<&'a str, O, Error> {
    move |inp| {
        let (inp, res) = f(inp)?;
        // TODO: Also be satisfied if a peek results in a paren
        let (inp, _) = some_whitespace(inp)?;
        Ok((inp, res))
    }
}

pub fn lexeme<'a, O>(f: impl Fn(&'a str) -> IResult<&'a str, O, Error>) -> impl Fn(&'a str) -> IResult<&'a str, O, Error> {
    move |inp| {
        let (inp, res) = f(inp)?;
        let (inp, _) = any_whitespace(inp)?;
        Ok((inp, res))
    }
}

pub fn surrounded<'a, O>(l: &'a str, r: &'a str, f: impl Fn(&'a str) -> IResult<&'a str, O, Error>) -> impl Fn(&'a str) -> IResult<&'a str, O, Error> {
    move |inp| {
        let (inp, _) = lexeme(tag(l))(inp)?;
        cut(|inp| {
            let (inp, res) = f(inp)?;
            let (inp, _) = lexeme(tag(r))(inp)?;
            Ok((inp, res))
        })(inp)
    }
}
