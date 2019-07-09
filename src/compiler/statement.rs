use crate::ast::*;
use crate::bytecode::*;

use super::errors::Compiler;
use super::phases::{PreInterns, PreProcedure};

use Instruction::*;

impl Statement {
    pub fn compile(self, pp: &mut PreProcedure, it: &mut PreInterns) -> Compiler<()> {
        match self {
            Statement::If(cond, bl_then, obl_else) => {
                let lb_then = pp.create_label();
                let lb_else = pp.create_label();
                let lb_done = pp.create_label();

                cond.compile(pp, it, lb_else);

                pp.anchor_label(lb_then)?;
                bl_then.compile(pp, it)?;

                match obl_else {
                    Some(bl_else) => {
                        pp.push(Jump(lb_done));
                        pp.anchor_label(lb_else)?;
                        bl_else.compile(pp, it)?;
                    }
                    None => {
                        pp.anchor_label(lb_else)?;
                    }
                }

                pp.anchor_label(lb_done)?;
            }
            Statement::Destructure(lhs, rhs) => {
                rhs.compile(pp, it);
                lhs.compile_destructure(pp, it, false); // don't unwind on fail, since we aren't in a conditional situation
            }
            Statement::Ret(expression) => {
                expression.compile(pp, it);
                pp.push(Ret);
            }
        };
        Ok(())
    }
}