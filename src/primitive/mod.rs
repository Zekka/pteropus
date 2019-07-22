use std::collections::BTreeSet;
use crate::interns::Intern;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Functor<TInt>(pub TInt, pub usize);

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Value {
    Integer(i64),
    Bool(bool),

    Compound(Intern, Vec<Value>),
    Vector(Vec<Value>),
    Set(BTreeSet<Value>),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Local(pub usize);


#[derive(Clone, Copy, Debug)]
pub enum Operand {
    Integer(i64),
    Bool(bool),
}
