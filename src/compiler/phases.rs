use std::collections::HashMap;
use super::errors::*;

use crate::bytecode::*;
use crate::executable::{Interns, Procedure};
use crate::prim::Functor;

pub struct PreProcedure {
    functor: Functor<InternIx>,

    instructions: Vec<IxInstruction>,
    anchor_labels: HashMap<LabelIx, usize>,
    next_label: LabelIx,
    pub local_name_to_ix: HashMap<String, LocalIx>,
    next_local: LocalIx,
}

#[derive(Debug)]
pub struct PreInterns {
    pub string: Vec<String>,
    pub intern: HashMap<String, usize>,
}

impl PreProcedure {
    pub fn new(functor: Functor<InternIx>) -> PreProcedure {
        PreProcedure {
            functor: functor,

            instructions: vec![],
            anchor_labels: HashMap::new(),
            next_label: LabelIx(0),
            local_name_to_ix: HashMap::new(),
            next_local: LocalIx(0),
        }
    }

    pub fn local(&mut self, s: &str) -> LocalIx {
        match self.local_name_to_ix.get(s) {
            Some(i) => { return *i; }
            None => {}
        };
        let nx = self.next_local;
        self.next_local.0 += 1;
        self.local_name_to_ix.insert(s.to_owned(), nx);
        return nx;
    }

    pub fn push(&mut self, i: IxInstruction) {
        self.instructions.push(i);
    }

    pub fn create_label(&mut self) -> LabelIx {
        let nx = self.next_label;
        self.next_label.0 += 1;
        return nx;
    }

    pub fn anchor_label(&mut self, lix: LabelIx) -> Compiler<()> {
        match self.anchor_labels.get(&lix) {
            Some(_) => {
                Err(Error::AlreadyAnchored(lix.0))
            }
            None => {
                self.anchor_labels.insert(lix, self.instructions.len());
                Ok(())
            }
        }
    }

    pub fn compile(mut self) -> Compiler<Procedure> {
        let mut is2 = vec![];
        for inst in self.instructions.drain(..) {
            use Instruction::*;
            is2.push(match inst {
                Push(op) => Push(op),
                Equals => Equals,
                Assert => Assert,
                Set(loc) => Set(loc.0),
                SetOr(loc) => SetOr(loc.0),
                Get(loc) => Get(loc.0),
                Jump(l) => Jump(
                    if let Some(anc) = self.anchor_labels.get(&l) { *anc } 
                    else { return Err(Error::NotAnchored(l.0)) }
                ),
                JumpNo(l) => JumpNo(
                    if let Some(anc) = self.anchor_labels.get(&l) { *anc } 
                    else { return Err(Error::NotAnchored(l.0)) }
                ),
                Pop => Pop,
                Ret => Ret,
                Call => Call,
                IsVec => IsVec,
                DestructCompound(Functor(f, a)) => DestructCompound(Functor(f.0, a)),
                Destruct(s) => Destruct(s),
                ConstructCompound(Functor(f, a)) => ConstructCompound(Functor(f.0, a)),
                ConstructVector(s) => ConstructVector(s),
                ConstructSet(s) => ConstructSet(s),

                Mark(l) => Mark(
                    if let Some(anc) = self.anchor_labels.get(&l) { *anc } 
                    else { return Err(Error::NotAnchored(l.0)) }
                ), 
                Unmark => Unmark, 
                UnwindNo => UnwindNo,

                Mul => Mul, Div => Div,
                Add => Add, Subtract => Subtract,

                Le => Le, Ge => Ge,
                Lt => Lt, Gt => Gt,
                Eq => Eq, Ne => Ne,
            })
        };
        Ok(Procedure {
            functor: Functor(self.functor.0 .0, self.functor.1),
            vars: self.next_local.0,
            instructions: is2,
        })
    }
}

impl PreInterns {
    pub fn new() -> PreInterns {
        PreInterns {
            intern: HashMap::new(),
            string: vec![],
        }
    }

    pub fn extend(module: &crate::executable::Module) -> PreInterns {
        // TODO: Don't clone these -- instead provide an alt PreInterns-y type that takes a ref
        PreInterns {
            intern: module.interns.intern.clone(),
            string: module.interns.string.clone(),
        }
    }

    pub fn to_intern(&mut self, s: &str) -> InternIx {
        match self.intern.get(s) {
            Some(i) => { return InternIx(*i); }
            None => {}
        }
        let id = self.string.len();
        self.intern.insert(s.to_string(), id);
        self.string.push(s.to_string());
        return InternIx(id);
    }

    pub fn compile(self) -> Interns {
        Interns {
            string: self.string,
            intern: self.intern,
        }
    }
}