use crate::interns::Interns;
use crate::irs::ast1;
use crate::irs::instruction1;
use crate::irs::procedure1;
use crate::primitive::{Functor, Operand};

use procedure1::Procedure1;

use ast1::{BinOp, Expression};
use instruction1::Instruction1;

impl Expression {
    pub fn compile(self, it: &mut Interns, pp: &mut Procedure1) {
        use Instruction1::*;
        use Expression as E;
        match self {
            E::NoOp => {}

            E::IntLiteral(i) => {
                pp.push(Push(Operand::Integer(i)));
            }
            E::Variable(n) => {
                let loc = pp.local(&n);
                pp.push(Get(loc));
            }
            E::Call(box e) => {
                e.compile(it, pp);
                pp.push(Instruction1::Call);
            }
            E::Compound(s, mut ve) => {
                let n = ve.len();
                for i in ve.drain(..) {
                    i.compile(it, pp);
                }
                pp.push(ConstructCompound(Functor(it.intern(&s), n)));
            }
            E::Vector(mut ve) => {
                let n = ve.len();
                for i in ve.drain(..) {
                    i.compile(it, pp);
                }
                pp.push(ConstructVector(n));
            }
            E::Set(mut ve) => {
                let n = ve.len();
                for i in ve.drain(..) {
                    i.compile(it, pp);
                }
                pp.push(ConstructSet(n));
            }

            E::Binary(box _lhs, BinOp::And, box _rhs) => {
                panic!("can't compile And yet");
            }

            E::Binary(box _lhs, BinOp::Or, box _rhs) => {
                panic!("can't compile Or yet");
            }

            E::Binary(box lhs, op, box rhs) => {
                lhs.compile(it, pp);
                rhs.compile(it, pp);
                pp.push(match op {
                    BinOp::And => unreachable!(),
                    BinOp::Or => unreachable!(),

                    BinOp::Multiply => Mul,
                    BinOp::Divide => Div,
                    BinOp::Add => Add,
                    BinOp::Subtract => Subtract,

                    BinOp::Le => Le,
                    BinOp::Ge => Ge,
                    BinOp::Lt => Lt,
                    BinOp::Gt => Gt,
                    BinOp::Eq => Eq,
                    BinOp::Ne => Ne,
                });
            }
        }
    }
}
