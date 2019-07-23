use crate::errors::runtime::*;
use crate::irs::procedure2::Procedure2;

use crate::vm::bvalue::*;

#[derive(Debug)]
pub struct StackFrame<'bump, 'code> {
    // code
    pub c: &'code Procedure2,
    pub ip: usize,
    // abbreviated to make the runner shorter
    pub v: Vec<Option<SBV<'bump>>>, // vars
    pub s: Vec<SBV<'bump>>, // stack
}

impl<'bump, 'code> StackFrame<'bump, 'code> {
    pub fn new_on(c: &'code Procedure2) -> StackFrame<'bump, 'code> {
        let mut frame = StackFrame {
            c,
            ip: 0,
            v: Vec::with_capacity(c.vars),
            s: vec![],
        };

        for _ in 0..c.vars {
            frame.v.push(None);
        };

        return frame;
    }

    pub fn push(&mut self, v: SBV<'bump>) {
        self.s.push(v)
    }

    pub fn pop(&mut self) -> Runtime<SBV<'bump>> {
        match self.s.pop() {
            None => Err(Error::NoMoreValues),
            Some(x) => Ok(x),
        }
    }

    pub fn pop_num2(&mut self) -> Runtime<Num2> {
        match self.s.pop() {
            None => Err(Error::NoMoreValues),
            Some(n1) => match self.s.pop() {
                None => Err(Error::NoMoreValues),
                Some(n2) => match (n1.as_immut(), n2.as_immut()) {
                    // RHS is on top
                    (BValue::Integer(i1), BValue::Integer(i2)) => Ok(Num2::Integer(*i2, *i1)),
                    _ => Err(Error::NotNumbers)
                }
            }
        }
    }

    pub fn peek(&self) -> Runtime<&SBV<'bump>> {
        match self.s.len() {
            0 => Err(Error::NoMoreValues),
            _ => Ok(&self.s[self.s.len() - 1]),
        }
    }
}


pub enum Num2 {
    Integer(i64, i64),
}


