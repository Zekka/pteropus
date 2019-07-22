use crate::errors::compiler::{Compiler, Error};
use crate::interns::Intern;
use crate::primitive::{Functor, Local};

use super::instruction1;
use super::instruction2;
use super::procedure2::Procedure2;

use instruction1::{Instruction1, Label};
use instruction2::{Instruction2, Ip};

use std::collections::HashMap;

pub struct Procedure1 {
    functor: Functor<Intern>,

    instructions: Vec<Instruction1>,
    anchor_labels: HashMap<Label, Ip>,
    next_label: Label,
    local_name_to_ix: HashMap<String, Local>,
    next_local: Local,
}

impl Procedure1 {
    pub fn new(functor: Functor<Intern>) -> Self {
        Procedure1 {
            functor,

            instructions: vec![],
            anchor_labels: HashMap::new(),
            next_label: Label(0),
            local_name_to_ix: HashMap::new(),
            next_local: Local(0),
        }
    }

    pub fn local_name_to_ix(&self) -> &HashMap<String, Local> {
        &self.local_name_to_ix
    }

    pub fn local(&mut self, s: &str) -> Local {
        match self.local_name_to_ix.get(s) {
            Some(i) => { return *i; }
            None => {}
        };
        let nx = self.next_local;
        self.next_local.0 += 1;
        self.local_name_to_ix.insert(s.to_owned(), nx);
        return nx;
    }

    pub fn push(&mut self, i: Instruction1) {
        self.instructions.push(i);
    }

    pub fn create_label(&mut self) -> Label {
        let nx = self.next_label;
        self.next_label.0 += 1;
        return nx;
    }

    pub fn anchor_label(&mut self, lix: Label) -> Compiler<()> {
        match self.anchor_labels.get(&lix) {
            Some(_) => {
                Err(Error::AlreadyAnchored(lix.0))
            }
            None => {
                self.anchor_labels.insert(lix, Ip(self.instructions.len()));
                Ok(())
            }
        }
    }

    pub fn compile(mut self) -> Compiler<Procedure2> {
        let mut is2 = vec![];
        for inst in self.instructions.drain(..) {
            use Instruction1 as A;
            use Instruction2 as B;
            is2.push(match inst {
                A::Push(op) => B::Push(op),
                A::Equals => B::Equals,
                A::EqualsOperandAssert(o) => B::EqualsOperandAssert(o),
                A::Assert => B::Assert,
                A::Set(loc) => B::Set(loc),
                A::SetAssert(loc) => B::SetAssert(loc),
                A::Get(loc) => B::Get(loc),
                A::Jump(l) => B::Jump(
                    if let Some(anc) = self.anchor_labels.get(&l) { *anc }
                    else { return Err(Error::NotAnchored(l.0)) }
                ),
                A::JumpNo(l) => B::JumpNo(
                    if let Some(anc) = self.anchor_labels.get(&l) { *anc }
                    else { return Err(Error::NotAnchored(l.0)) }
                ),
                A::Pop => B::Pop,
                A::Ret => B::Ret,
                A::Call => B::Call,

                A::Mark(l, keep_on_failure) => B::Mark(
                    if let Some(anc) = self.anchor_labels.get(&l) { *anc }
                    else { return Err(Error::NotAnchored(l.0)) },
                    keep_on_failure
                ),
                A::Unmark => B::Unmark,
                A::DestructCompound(Functor(f, a)) => B::DestructCompound(Functor(f, a)),
                A::DestructVector(s) => B::DestructVector(s),
                A::Destruct(s) => B::Destruct(s),
                A::ConstructCompound(Functor(f, a)) => B::ConstructCompound(Functor(f, a)),
                A::ConstructVector(s) => B::ConstructVector(s),
                A::ConstructSet(s) => B::ConstructSet(s),

                A::Mul => B::Mul, A::Div => B::Div,
                A::Add => B::Add, A::Subtract => B::Subtract,

                A::Le => B::Le, A::Ge => B::Ge,
                A::Lt => B::Lt, A::Gt => B::Gt,
                A::Eq => B::Eq, A::Ne => B::Ne,
            })
        };
        Ok(Procedure2 {
            functor: Functor(self.functor.0, self.functor.1),
            vars: self.next_local.0,
            instructions: is2,
        })
    }
}
