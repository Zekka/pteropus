use crate::errors::runtime::*;
use crate::irs::procedure2::Procedure2;
use crate::primitive::Value;

#[derive(Debug)]
pub struct StackFrame<'code> {
    // code
    pub c: &'code Procedure2,
    pub ip: usize,
    // abbreviated to make the runner shorter
    pub v: Vec<Option<Value>>, // vars
    pub s: Vec<Value>, // stack
}

impl<'code> StackFrame<'code> {
    pub fn new_on(c: &'code Procedure2) -> StackFrame<'code> {
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

    pub fn push(&mut self, v: Value) {
        self.s.push(v)
    }

    pub fn pop(&mut self) -> Runtime<Value> {
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
                Some(n2) => match (n1, n2) {
                    // RHS is on top
                    (Value::Integer(i1), Value::Integer(i2)) => Ok(Num2::Integer(i2, i1)),
                    _ => Err(Error::NotNumbers)
                }
            }
        }
    }

    pub fn peek(&self) -> Runtime<&Value> {
        match self.s.len() {
            0 => Err(Error::NoMoreValues),
            _ => Ok(&self.s[self.s.len() - 1]),
        }
    }
}


pub enum Num2 {
    Integer(i64, i64),
}


