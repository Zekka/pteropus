use super::*;

use nom;

named!(fmodule<&str, Module, Error>, terminated!(module, eof!()));


pub fn parse_module(s: &str) -> Result<Module, nom::Err<Error>> {
    let (s, _) = any_whitespace(s)?;
    match fmodule(s) {
        Ok(("", module)) => Ok(module),
        Ok((s, _)) => panic!("input not completed: {}", s),
        Err(e) => Err(e),
    }
}


named!(fprocedure<&str, Procedure, Error>, terminated!(procedure, eof!()));


pub fn parse_procedure(s: &str) -> Result<Procedure, nom::Err<Error>> {
    let (s, _) = any_whitespace(s)?;
    match fprocedure(s) {
        Ok(("", function)) => Ok(function),
        Ok((s, _)) => panic!("input not completed: {}", s),
        Err(e) => Err(e),
    }
}


named!(fstatement<&str, Statement, Error>, terminated!(statement, eof!()));


pub fn parse_repl_statement(s: &str) -> Result<Statement, nom::Err<Error>> {
    let (s, _) = any_whitespace(s)?;
    match fstatement(s) {
        Ok(("", statement)) => Ok(statement),
        Ok((s, _)) => panic!("input not completed: {}", s),
        Err(e) => Err(e),
    }
}