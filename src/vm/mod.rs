mod errors;

use crate::bytecode::{Instruction, Operand};
use crate::executable::{Module, Procedure};
use errors::{Error, Runtime};

use crate::prim::Functor;

use bit_set::BitSet;
use std::collections::btree_set::BTreeSet;
use std::iter::FromIterator;

#[derive(Debug)]
pub enum VM<'a> {
    Updating,
    Running(Running<'a>),
    Succeeded(Value),
    Failed(Error),
}

#[derive(Debug)]
pub struct Running<'a> {
    code: &'a Module,
    frames: Vec<StackFrame<'a>>
}

#[derive(Debug)]
struct StackFrame<'a> {
    code: &'a Procedure,
    ip: usize,
    vars: Vec<Option<Value>>,
    stack: Vec<Value>,

    mark: Option<MarkRegister>,
    touched: BitSet,
}

#[derive(Clone, Copy, Debug)]
struct MarkRegister {
    stack_size: usize,
    ip: usize,
}

// TODO: Factor out Clone when possible (can't get rid of it completely, probably)
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Value {
    Integer(i64),
    Bool(bool),

    Compound(usize, Vec<Value>),
    Vector(Vec<Value>),
    Set(BTreeSet<Value>),
}

enum Num2 {
    Integer(i64, i64),
}

impl<'a> VM<'a> {
    pub fn go(code: &'a Module, call: Value) -> Runtime<Self> {
        let running = Running { code: code, frames: vec![] };
        running.call(call)
    }

    pub fn update(&mut self) {
        let mut tar = VM::Updating;
        ::std::mem::swap(self, &mut tar);

        *self = match tar {
            VM::Updating => { VM::Failed(Error::UpdatedWhileUpdating) }
            VM::Running(running) => {
                match running.update() {
                    Err(e) => { VM::Failed(e) }
                    Ok(v) => { v }
                }
            }
            VM::Succeeded(succeeded) => { VM::Succeeded(succeeded) }
            VM::Failed(failed) => { VM::Failed(failed) }
        }
    }

    pub fn is_running(&self) -> bool {
        match self {
            VM::Running(_) => { true }
            _ => { false }
        }
    }
}

impl<'a> Running<'a> {
    fn call(mut self, call: Value) -> Runtime<VM<'a>> {
        let code: &Procedure = match &call {
            Value::Compound(intern, args) => {
                // ZEKKA NOTE: Holy shit, this clone is inefficient. Terrible!
                // Replace it later!
                match self.code.procedures.get(&Functor(*intern, args.len())) {
                    None => { return Err(Error::NoSuchProcedure); }
                    Some(code) => code
                }
            }
            _ => {
                return Err(Error::CallNotCompound);
            }
        };

        let mut frame = StackFrame {
            code: code,
            ip: 0,
            vars: Vec::with_capacity(code.vars),
            stack: vec![call],
            mark: None,
            touched: BitSet::with_capacity(code.vars),
        };

        for _ in 0..code.vars {
            frame.vars.push(None);
        }

        self.frames.push(frame);
        Ok(VM::Running(self))
    }

    fn update(mut self) -> Runtime<VM<'a>> {
        if self.frames.len() == 0 { return Err(Error::NoMoreFrames); }

        let sp = self.frames.len() - 1;
        let ip = self.frames[sp].ip;

        if ip >= self.frames[sp].code.instructions.len() { return Err(Error::OutOfCode); }
        self.frames[sp].ip += 1;

        use Instruction::*;
        println!("- {:?} {:?} {:?}", self.frames[sp].code.instructions[ip], self.frames[sp].stack, self.frames[sp].touched);
        match self.frames[sp].code.instructions[ip].clone() {
            Push(Operand::Integer(i)) => {
                self.frames[sp].stack.push(Value::Integer(i));
                Ok(VM::Running(self))
            }
            Push(Operand::Bool(b)) => {
                self.frames[sp].stack.push(Value::Bool(b));
                Ok(VM::Running(self))
            }
            Equals => {
                let s1 = pop(&mut self.frames[sp].stack)?;
                let s2 = pop(&mut self.frames[sp].stack)?;
                self.frames[sp].stack.push(Value::Bool(s1 == s2));
                Ok(VM::Running(self))
            }
            Assert => {
                let s1 = pop(&mut self.frames[sp].stack)?;
                match s1 {
                    Value::Bool(true) => {},
                    Value::Bool(false) => { return Err(Error::AssertionFailed); }
                    _ => return Err(Error::ConditionalWrongType)
                }
                Ok(VM::Running(self))
            }
            SetOr(vp) => {
                let s1 = pop(&mut self.frames[sp].stack)?;
                let to_push = match &self.frames[sp].vars[vp] {
                    None => { 
                        self.frames[sp].vars[vp] = Some(s1);
                        self.frames[sp].touched.insert(vp);
                        true
                    }
                    Some(x) => {
                        x == &s1
                    }
                };
                self.frames[sp].stack.push(Value::Bool(to_push));
                
                Ok(VM::Running(self))
            }
            Get(vp) => {
                let to_push = match &self.frames[sp].vars[vp] {
                    None => { return Err(Error::GetUnset) }
                    Some(x) => { x.clone() }
                };
                self.frames[sp].stack.push(to_push);
                Ok(VM::Running(self))
            }
            Jump(new_ip) => { 
                self.frames[sp].ip = new_ip;
                Ok(VM::Running(self))
            }
            JumpNo(new_ip) => { 
                let s1 = pop(&mut self.frames[sp].stack)?;
                match s1 {
                    Value::Bool(true) => {},
                    Value::Bool(false) => { 
                        self.frames[sp].ip = new_ip 
                    }
                    _ => return Err(Error::ConditionalWrongType)
                };
                Ok(VM::Running(self))
            }
            Ret => {
                let rval = pop(&mut self.frames[sp].stack)?;

                if self.frames.len() == 1 {
                    return Ok(VM::Succeeded(rval));
                }
                self.frames[sp - 1].stack.push(rval);
                self.frames.pop();
                Ok(VM::Running(self))
            }
            Call => {
                let invocation = pop(&mut self.frames[sp].stack)?;
                self.call(invocation)
            }
            IsVec => {
                let s1 = peek(&mut self.frames[sp].stack)?;
                match s1 {
                    Value::Vector(_) => self.frames[sp].stack.push(Value::Bool(true)),
                    _ => self.frames[sp].stack.push(Value::Bool(false))
                };
                Ok(VM::Running(self))
            }

            DestructCompound(f) => {
                let s1 = pop(&mut self.frames[sp].stack)?;
                match s1 {
                    Value::Compound(intern, mut args) if intern == f.0 && args.len() == f.1 => {
                        for arg in args.drain(..).rev() {
                            self.frames[sp].stack.push(arg);
                        };
                        self.frames[sp].stack.push(Value::Bool(true));
                        Ok(VM::Running(self))
                    }
                    _ => {
                        self.frames[sp].stack.push(Value::Bool(false));
                        Ok(VM::Running(self))
                    }
                }
            }

            Mark(l) => {
                match self.frames[sp].mark {
                    None => {
                        self.frames[sp].mark = Some(MarkRegister { stack_size: self.frames[sp].stack.len(), ip: l })
                    }
                    Some(_) => { return Err(Error::CantMarkTwice); }
                };
                self.frames[sp].touched.clear();
                Ok(VM::Running(self))
            }

            Unmark => {
                match self.frames[sp].mark {
                    None => {
                        return Err(Error::UnmarkMustBeMarked);
                    }
                    Some(mk) => { 
                        if self.frames[sp].stack.len() != mk.stack_size {
                            return Err(Error::UnmarkWrongStackSize);
                        }
                        self.frames[sp].mark = None;
                        self.frames[sp].touched.clear();
                    }
                };
                Ok(VM::Running(self))
            }

            UnwindNo => {
                let s1 = pop(&mut self.frames[sp].stack)?;
                let mk = match self.frames[sp].mark {
                    None => { return Err(Error::UnwindMustBeMarked); }
                    Some(mk) => mk,
                };
                match s1 {
                    Value::Bool(true) => {},
                    Value::Bool(false) => { 
                        if self.frames[sp].stack.len() < mk.stack_size {
                            return Err(Error::UnwindStackTooSmall);
                        }

                        // jump to new address
                        self.frames[sp].ip = mk.ip;

                        // rewind to previous state
                        self.frames[sp].stack.drain(mk.stack_size..);
                        let frame = &mut self.frames[sp];
                        for vp in frame.touched.iter() {
                            frame.vars[vp] = None;
                        }

                        // done rewinding!
                        self.frames[sp].mark = None;
                        self.frames[sp].touched.clear();
                    }
                    _ => return Err(Error::ConditionalWrongType)
                };
                Ok(VM::Running(self))
            }

            Destruct(sz) => {
                let s1 = pop(&mut self.frames[sp].stack)?;
                match s1 {
                    Value::Compound(_, mut args) if args.len() == sz => {
                        for arg in args.drain(..).rev() {
                            self.frames[sp].stack.push(arg);
                        };
                        self.frames[sp].stack.push(Value::Bool(true));
                        Ok(VM::Running(self))
                    }
                    Value::Vector(mut args) if args.len() == sz => {
                        for arg in args.drain(..).rev() {
                            self.frames[sp].stack.push(arg);
                        };
                        self.frames[sp].stack.push(Value::Bool(true));
                        Ok(VM::Running(self))
                    }
                    // A set can't be destructed
                    _ => {
                        self.frames[sp].stack.push(Value::Bool(false));
                        Ok(VM::Running(self))
                    }
                }
            }

            ConstructCompound(f) => {
                let len = self.frames[sp].stack.len();
                if len < f.1 { return Err(Error::NoMoreValues); }
                let values = self.frames[sp].stack.drain((len - f.1)..len).collect::<Vec<Value>>();
                self.frames[sp].stack.push(Value::Compound(f.0, values));
                Ok(VM::Running(self))
            }

            ConstructVector(sz) => {
                let len = self.frames[sp].stack.len();
                if len < sz { return Err(Error::NoMoreValues); }
                let values = self.frames[sp].stack.drain((len - sz)..len).collect::<Vec<Value>>();
                self.frames[sp].stack.push(Value::Vector(values));
                Ok(VM::Running(self))
            }

            ConstructSet(sz) => {
                let len = self.frames[sp].stack.len();
                if len < sz { return Err(Error::NoMoreValues); }
                let values = BTreeSet::from_iter(self.frames[sp].stack.drain((len - sz)..len).rev());
                self.frames[sp].stack.push(Value::Set(values));
                Ok(VM::Running(self))
            }

            Mul => {
                let top = pop_num2(&mut self.frames[sp].stack)?;
                self.frames[sp].stack.push(match top {
                    Num2::Integer(i1, i2) => Value::Integer(i1 * i2),
                });
                Ok(VM::Running(self))
            }

            Div => {
                let top = pop_num2(&mut self.frames[sp].stack)?;
                self.frames[sp].stack.push(match top {
                    Num2::Integer(i1, i2) => Value::Integer(i1 / i2),
                });
                Ok(VM::Running(self))
            }

            Add => {
                let top = pop_num2(&mut self.frames[sp].stack)?;
                self.frames[sp].stack.push(match top {
                    Num2::Integer(i1, i2) => Value::Integer(i1 + i2),
                });
                Ok(VM::Running(self))
            }

            Subtract => {
                let top = pop_num2(&mut self.frames[sp].stack)?;
                self.frames[sp].stack.push(match top {
                    Num2::Integer(i1, i2) => Value::Integer(i1 - i2),
                });
                Ok(VM::Running(self))
            }

            Le => {
                let top = pop_num2(&mut self.frames[sp].stack)?;
                self.frames[sp].stack.push(match top {
                    Num2::Integer(i1, i2) => Value::Bool(i1 <= i2),
                });
                Ok(VM::Running(self))
            }

            Ge => {
                let top = pop_num2(&mut self.frames[sp].stack)?;
                self.frames[sp].stack.push(match top {
                    Num2::Integer(i1, i2) => Value::Bool(i1 >= i2),
                });
                Ok(VM::Running(self))
            }

            Lt => {
                let top = pop_num2(&mut self.frames[sp].stack)?;
                self.frames[sp].stack.push(match top {
                    Num2::Integer(i1, i2) => Value::Bool(i1 < i2),
                });
                Ok(VM::Running(self))
            }

            Gt => {
                let top = pop_num2(&mut self.frames[sp].stack)?;
                self.frames[sp].stack.push(match top {
                    Num2::Integer(i1, i2) => Value::Bool(i1 > i2),
                });
                Ok(VM::Running(self))
            }

            Eq => {
                let top = pop_num2(&mut self.frames[sp].stack)?;
                self.frames[sp].stack.push(match top {
                    Num2::Integer(i1, i2) => Value::Bool(i1 == i2),
                });
                Ok(VM::Running(self))
            }

            Ne => {
                let top = pop_num2(&mut self.frames[sp].stack)?;
                self.frames[sp].stack.push(match top {
                    Num2::Integer(i1, i2) => Value::Bool(i1 != i2),
                });
                Ok(VM::Running(self))
            }
        }
    }
}

fn pop(v: &mut Vec<Value>) -> Runtime<Value> {
    match v.pop() {
        None => Err(Error::NoMoreValues),
        Some(x) => Ok(x),
    }
}

fn pop_num2(v: &mut Vec<Value>) -> Runtime<Num2> {
    match v.pop() {
        None => Err(Error::NoMoreValues),
        Some(n1) => match v.pop() {
            None => Err(Error::NoMoreValues),
            Some(n2) => match (n1, n2) {
                // RHS is on top
                (Value::Integer(i1), Value::Integer(i2)) => Ok(Num2::Integer(i2, i1)),
                _ => Err(Error::NotNumbers)
            }
        }
    }
}


fn peek<'a>(v: &'a Vec<Value>) -> Runtime<&Value> {
    match v.len() {
        0 => Err(Error::NoMoreValues),
        _ => Ok(&v[v.len() - 1]),
    }
}