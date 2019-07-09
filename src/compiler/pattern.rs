use crate::ast::*;
use crate::bytecode::*;
use crate::prim::Functor;

use super::phases::{PreInterns, PreProcedure};

use Instruction::*;

impl Pattern {
    pub fn compile_destructure(self, pp: &mut PreProcedure, it: &mut PreInterns, unwind_on_fail: bool) {
        // there is already a thing on the stack which is the target of the destructure op
        match self {
            Pattern::IntLiteral(i) => {
                pp.push(Push(Operand::Integer(i)));
                pp.push(Equals);
                pp.push(if unwind_on_fail { UnwindNo } else { Assert });
            }
            Pattern::Variable(n) => {
                let loc = pp.local(&n);
                pp.push(SetOr(loc));
                pp.push(if unwind_on_fail { UnwindNo } else { Assert });
            }
            Pattern::Compound(s, mut v) => {
                pp.push(DestructCompound(Functor(it.to_intern(&s), v.len())));
                pp.push(if unwind_on_fail { UnwindNo } else { Assert });

                for i in v.drain(..) {
                    i.compile_destructure(pp, it, unwind_on_fail);
                }
            }
            Pattern::WcCompound(mut v) => {
                pp.push(Destruct(v.len()));
                pp.push(if unwind_on_fail { UnwindNo } else { Assert });

                for i in v.drain(..) {
                    i.compile_destructure(pp, it, unwind_on_fail);
                }
            }
            Pattern::Vector(mut v) => {
                pp.push(IsVec);
                pp.push(if unwind_on_fail { UnwindNo } else { Assert });
                pp.push(Destruct(v.len()));
                pp.push(if unwind_on_fail { UnwindNo } else { Assert });

                for i in v.drain(..) {
                    i.compile_destructure(pp, it, unwind_on_fail);
                }
            }
        }
    }
}