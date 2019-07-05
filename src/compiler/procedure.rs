use crate::ast::*;
use crate::prim::Functor;

use super::errors::*;
use super::phases::{PreInterns, PreProcedure};
use crate::executable::Procedure;

impl crate::ast::Procedure {
    pub fn compile(self, it: &mut PreInterns) -> Compiler<Procedure> {
        let mut pp = PreProcedure::new(Functor(it.to_intern(&self.name), self.args.len()));

        // compile args block
        // it's assumed the top will always be on the stack
        let artificial_lhs = Pattern::WcCompound(self.args);
        let first_statement = Statement::Destructure(artificial_lhs, Expression::NoOp);
        first_statement.compile(&mut pp, it)?;
        self.body.compile(&mut pp, it)?;

        pp.compile()
    }
}