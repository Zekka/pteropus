use crate::interns::Intern;
use crate::primitive::Functor;

use super::instruction2::Instruction2;

#[derive(Debug)]
pub struct Procedure2 {
    pub functor: Functor<Intern>,
    pub instructions: Vec<Instruction2>,
    pub vars: usize,
}
