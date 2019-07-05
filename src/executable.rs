use std::collections::HashMap;
use super::bytecode::RawInstruction;
use crate::prim::Functor;

#[derive(Debug)]
pub struct Module {
    pub interns: Interns,
    pub procedures: HashMap<Functor<usize>, Procedure>,
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