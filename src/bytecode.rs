use crate::prim::Functor;

#[derive(Clone, Debug)]
pub enum Operand {
    Integer(i64),
    Bool(bool),
}

// TODO: 
//   Make cloning basically free
//   by interning functor heads
#[derive(Clone, Debug)]
pub enum Instruction<TLoc, TLab, TInt> {
    Push(Operand),
    // push true if s1 == s2
    Equals, 
    // fail if s1 != true 
    Assert, 
    // *s1 = s2, no matter what
    Set(TLoc), 
    // *s1 = s2; push false if s1 is set and != s2, otherwise push true
    SetOr(TLoc), 
    // *s1
    Get(TLoc),
    // jump to s1
    Jump(TLab), 
    // jump to s1 if s2 is false
    JumpNo(TLab),
    // return s1
    Ret,
    // call s1
    Call,

    // peeks at stack top, pushes true if it's a vec and false if it's not
    IsVec,
    // use desired number of args, fail if not the same as the number of args
    // use name, fail if this is not the name of the composite
    // on failure: push false
    // on true: push args to stack in reverse order, then push true
    DestructCompound(Functor<TInt>),

    // use desired number of args, fail if not the same as the number of args
    // on failure: push false
    // on true: push args to stack in reverse order, then push true
    Destruct(usize),

    // use desired number of args, pop desired name, reconstruct in reverse order
    ConstructCompound(Functor<TInt>),
    // use desired number of args, reconstruct in reverse order
    ConstructVector(usize),
    ConstructSet(usize),

    // Destructuring error handling
    Mark(TLab), Unmark, UnwindNo,

    // Binop stuff
    Mul, Div,
    Add, Subtract,

    Le, Ge,
    Lt, Gt,
    Eq, Ne,
}

pub type IxInstruction = Instruction<LocalIx, LabelIx, InternIx>;
pub type RawInstruction = Instruction<usize, usize, usize>;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct LocalIx(pub usize);

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct LabelIx(pub usize);

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct InternIx(pub usize);