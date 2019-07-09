use crate::ast::*;
use crate::bytecode::*;
use crate::prim::Functor;

use super::phases::{PreInterns, PreProcedure};

use Instruction::*;

impl Condition {
    pub fn compile(self, pp: &mut PreProcedure, it: &mut PreInterns, lb_else: LabelIx) {
        match self {
            Condition::Let(lhs, rhs) => {
                pp.push(Mark(lb_else));
                rhs.compile(pp, it);
                lhs.compile_destructure(pp, it, true); // unwind on fail, since we are in a conditional situation
                pp.push(Unmark);
            }
            Condition::Bare(xp) => {
                xp.compile(pp, it);
                pp.push(JumpNo(lb_else));
            }
        }
    }
}

impl Expression {
    pub fn compile(self, pp: &mut PreProcedure, it: &mut PreInterns) {
        use Expression::*;
        match self {
            NoOp => {}

            IntLiteral(i) => {
                pp.push(Push(Operand::Integer(i)));
            }
            Variable(n) => {
                let loc = pp.local(&n);
                pp.push(Get(loc));
            }
            Call(box e) => {
                e.compile(pp, it);
                pp.push(Instruction::Call);
            }
            Compound(s, mut ve) => {
                let n = ve.len();
                for i in ve.drain(..) {
                    i.compile(pp, it);
                }
                pp.push(ConstructCompound(Functor(it.to_intern(&s), n)));
            }
            Vector(mut ve) => {
                let n = ve.len();
                for i in ve.drain(..) {
                    i.compile(pp, it);
                }
                pp.push(ConstructVector(n));
            }
            Set(mut ve) => {
                let n = ve.len();
                for i in ve.drain(..) {
                    i.compile(pp, it);
                }
                pp.push(ConstructSet(n));
            }

            Binary(box _lhs, BinOp::And, box _rhs) => {
                panic!("can't compile And yet");
            }

            Binary(box _lhs, BinOp::Or, box _rhs) => {
                panic!("can't compile Or yet");
            }

            Binary(box lhs, op, box rhs) => {
                lhs.compile(pp, it);
                rhs.compile(pp, it);
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