use crate::interns::Interns;
use crate::irs::ast1;
use crate::irs::instruction1;
use crate::irs::procedure1;

use ast1::Condition;
use procedure1::Procedure1;
use instruction1::{Instruction1, Label};

impl Condition {
    pub fn compile(self, it: &mut Interns, pp: &mut Procedure1, lb_else: Label) {
        use Instruction1::*;
        match self {
            Condition::Let(lhs, rhs) => {
                rhs.compile(it, pp);
                pp.push(Mark(lb_else, false));
                lhs.compile_destructure(it, pp);
                pp.push(Unmark);
            }
            Condition::Bare(xp) => {
                xp.compile(it, pp);
                pp.push(JumpNo(lb_else));
            }
        }
    }
}
