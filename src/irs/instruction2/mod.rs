use crate::primitive::{Functor, Local, Operand};
use crate::interns::Intern;

#[derive(Clone, Copy, Debug)]
pub enum Instruction2 {
    Push(Operand),

    Set(Local), SetAssert(Local),
    Get(Local),

    Assert,
    Jump(Ip), JumpNo(Ip),

    Pop, Ret, Call,

    // bool: whether to keep the stack item on a failure
    Mark(Ip, bool), Unmark,
    DestructCompound(Functor<Intern>), DestructVector(usize), Destruct(usize),
    ConstructCompound(Functor<Intern>), ConstructVector(usize), ConstructSet(usize),

    Equals, EqualsOperandAssert(Operand),

    Mul, Div, Add, Subtract,
    Le, Ge, Lt, Gt, Eq, Ne,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Ip(pub usize);
