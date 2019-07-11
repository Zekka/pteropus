use std::collections::HashMap;
use super::bytecode::RawInstruction;
use crate::prim::Functor;
use crate::vm::Value;

use std::fmt;

#[derive(Debug)]
pub struct Module {
    pub interns: Interns,
    pub procedures: HashMap<Functor<usize>, FFIProcedure>,
}

pub enum FFIProcedure {
    Native(Box<Fn(&Module, Value) -> Value>),
    Dynamic(Procedure),
}

impl fmt::Debug for FFIProcedure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FFIProcedure::Native(_) => { write!(f, "Native(:native code:)") }
            FFIProcedure::Dynamic(p) => { write!(f, "Dynapic({:?})", p) }
        }
    }
}

#[derive(Debug)]
pub struct Procedure {
    pub functor: Functor<usize>,
    pub instructions: Vec<RawInstruction>,
    pub vars: usize,
}

#[derive(Debug)]
pub struct Interns {
    pub string: Vec<String>,
    pub intern: HashMap<String, usize>,
}

impl Module {
    pub fn to_intern(&self, s: &str) -> Option<usize> {
        self.interns.intern.get(s).map(|i| *i)
    }

    pub fn from_intern<'a>(&'a self, i: usize) -> Option<&'a str> {
        self.interns.string.get(i).map(|i| i.as_str())
    }
}