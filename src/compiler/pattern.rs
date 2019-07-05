use crate::ast::*;
use crate::bytecode::*;
use crate::prim::Functor;

use super::phases::{PreInterns, PreProcedure};

use Instruction::*;

impl Pattern {
    // ZEKKA NOTE: 
    // - on_fail can't be a flavor of jump until we figure out how to clear the stack in those cases
    // - probably by having a flavor of jump that pops a fixed number of things off the stack, then making
    //   on_fail take the number of those to do
    //   
    //   or by providing mark/demark/unwind instructions, which will:
    //   - mark an unwind site (max of 1 at a time)
    //   - remove an unwind marker
    //   - unwind to the last unwind site (a stack address), then jump (an instruction pointer)
    //   - if no unwind address is set, `assert` (and fail as usual)
    //
    //   or just by having no early jumps?
    //     

    pub fn compile_destructure(self, pp: &mut PreProcedure, it: &mut PreInterns) { //, on_fail: IxInstruction) {
        let on_fail = Assert;
        // there is already a thing on the stack which is the target of the destructure op
        match self {
            Pattern::IntLiteral(i) => {
                pp.push(Push(Operand::Integer(i)));
                pp.push(Equals);
                pp.push(on_fail);
            }
            Pattern::Variable(n) => {
                let loc = pp.local(&n);
                pp.push(SetOr(loc));
                pp.push(on_fail);
            }
            Pattern::Compound(s, mut v) => {
                pp.push(DestructCompound(Functor(it.to_intern(&s), v.len())));
                pp.push(on_fail);

                for i in v.drain(..) {
                    i.compile_destructure(pp, it); //, on_fail);
                }
            }
            Pattern::WcCompound(mut v) => {
                pp.push(Destruct(v.len()));
                pp.push(on_fail);

                for i in v.drain(..) {
                    i.compile_destructure(pp, it); //, on_fail);
                }
            }
            Pattern::Vector(mut v) => {
                pp.push(IsVec);
                pp.push(on_fail.clone());
                pp.push(Destruct(v.len()));
                pp.push(on_fail);

                for i in v.drain(..) {
                    i.compile_destructure(pp, it); // , on_fail);
                }
            }
        }
    }
}