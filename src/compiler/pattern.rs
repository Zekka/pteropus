use crate::interns::Interns;
use crate::irs::ast1;
use crate::irs::instruction1;
use crate::irs::procedure1;
use crate::primitive::{Functor, Operand};

use ast1::Pattern;
use instruction1::Instruction1;
use procedure1::Procedure1;

impl Pattern {
    pub fn compile_destructure(self, it: &mut Interns, pp: &mut Procedure1) {
        // there is already a thing on the stack which is the target of the destructure op
        use Instruction1::*;

        match self {
            Pattern::IntLiteral(i) => {
                pp.push(EqualsOperandAssert(Operand::Integer(i)));
            }
            Pattern::Variable(n) => {
                let loc = pp.local(&n);
                pp.push(SetAssert(loc));
            }
            Pattern::Compound(s, mut v) => {
                pp.push(DestructCompound(Functor(it.intern(&s), v.len())));

                for i in v.drain(..) {
                    i.compile_destructure(it, pp);
                }
            }
            Pattern::WcCompound(mut v) => {
                pp.push(Destruct(v.len()));

                for i in v.drain(..) {
                    i.compile_destructure(it, pp);
                }
            }
            Pattern::Vector(mut v) => {
                pp.push(DestructVector(v.len()));

                for i in v.drain(..) {
                    i.compile_destructure(it, pp);
                }
            }
        }
    }
}
