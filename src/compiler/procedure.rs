use crate::interns::Interns;
use crate::irs::ast1;
use crate::irs::procedure1;
use crate::irs::procedure2;
use crate::primitive::Functor;

use crate::errors::compiler::*;

use ast1::{Expression, Pattern, Procedure, Statement};
use procedure1::Procedure1;
use procedure2::Procedure2;

impl Procedure {
    pub fn compile(self, it: &mut Interns) -> Compiler<Procedure2> {
        let mut pp = Procedure1::new(Functor(it.intern(&self.name), self.args.len()));

        // compile args block
        // it's assumed the top will always be on the stack
        let artificial_lhs = Pattern::WcCompound(self.args);
        let first_statement = Statement::Destructure(artificial_lhs, Expression::NoOp);
        first_statement.compile(it, &mut pp)?;
        self.body.compile(it, &mut pp)?;

        pp.compile()
    }
}
