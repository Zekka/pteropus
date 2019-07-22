use std::collections::HashMap;

use crate::interns::{Intern, Interns};
use crate::irs::procedure2::Procedure2;
use crate::primitive::{Functor, Value};

use std::fmt;

#[derive(Debug)]
pub struct Executable1 {
    pub procedures: HashMap<Functor<Intern>, FFIProcedure>,
}

pub enum FFIProcedure {
    Native(Box<Fn(&Interns, &Executable1, Value) -> Value>),
    Dynamic(Procedure2),
}

impl fmt::Debug for FFIProcedure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FFIProcedure::Native(_) => { write!(f, "Native(:native code:)") }
            FFIProcedure::Dynamic(p) => { write!(f, "Dynamic({:?})", p) }
        }
    }
}
