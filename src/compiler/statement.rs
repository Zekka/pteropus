use crate::interns::Interns;
use crate::irs::ast1;
use crate::irs::instruction1;
use crate::irs::procedure1;
use crate::irs::procedure2;
use crate::primitive::{Functor, Local, Operand};

use crate::errors::compiler::*;

use ast1::{Statement};
use instruction1::Instruction1;
use procedure1::Procedure1;
use procedure2::Procedure2;

impl Statement {
    pub fn compile_repl(self, it: &mut Interns) -> Compiler<(
        std::collections::HashMap<String, Local>,
        Procedure2,
    )> {
        let mut preprocedure = Procedure1::new(Functor(it.intern("repl"), 0));

        self.compile(it, &mut preprocedure)?;
        preprocedure.push(Instruction1::Push(Operand::Integer(1)));
        preprocedure.push(Instruction1::Ret);

        let mut names = std::collections::HashMap::new();
        for (k, v) in preprocedure.local_name_to_ix().iter() {
            names.insert(k.clone(), v.clone());
        }
        Ok((names, preprocedure.compile()?))
    }

    pub fn compile(self, it: &mut Interns, pp: &mut Procedure1) -> Compiler<()> {
        use Instruction1::*;
        match self {
            Statement::If(cond, bl_then, obl_else) => {
                let lb_then = pp.create_label();
                let lb_else = pp.create_label();
                let lb_done = pp.create_label();

                cond.compile(it, pp, lb_else);

                pp.anchor_label(lb_then)?;
                bl_then.compile(it, pp)?;

                match obl_else {
                    Some(bl_else) => {
                        pp.push(Jump(lb_done));
                        pp.anchor_label(lb_else)?;
                        bl_else.compile(it, pp)?;
                    }
                    None => {
                        pp.anchor_label(lb_else)?;
                    }
                }

                pp.anchor_label(lb_done)?;
            }
            Statement::Assign(lhs, rhs) => {
                rhs.compile(it, pp);
                let ix = pp.local(&lhs);
                pp.push(Set(ix));
            }
            Statement::Destructure(lhs, rhs) => {
                rhs.compile(it, pp);
                lhs.compile_destructure(it, pp); // don't unwind on fail, since we aren't in a conditional situation
            }
            Statement::Eval(expression) => {
                expression.compile(it, pp);
                pp.push(Pop);
            }
            Statement::Ret(expression) => {
                expression.compile(it, pp);
                pp.push(Ret);
            }
        };
        Ok(())
    }
}
