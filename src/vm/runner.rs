use super::VM;
use super::stackframe::*;

use crate::errors::runtime::*;
use crate::interns::Interns;
use crate::irs::executable1::{Executable1, FFIProcedure};
use crate::irs::instruction2::Instruction2;
use crate::irs::procedure2::Procedure2;
use crate::primitive::{Functor, Operand, Value};

use bit_set::BitSet;
use std::collections::btree_set::BTreeSet;
use std::iter::FromIterator;


#[derive(Debug)]
pub struct Runner<'a> {
    // code, frames
    // short names for terse implementations
    pub c: &'a Executable1,
    pub f: Vec<StackFrame<'a>>,
}


impl<'a> Runner<'a> {
    pub fn new(c: &Executable1) -> Runner {
        Runner {c, f: vec![]}
    }

    pub fn call(mut self, interns: &Interns, call: Value) -> Runtime<VM<'a>> {
        let c: &Procedure2 = match &call {
            Value::Compound(intern, args) => {
                match self.c.procedures.get(&Functor(*intern, args.len())) {
                    None => { return Err(Error::NoSuchProcedure); }
                    Some(FFIProcedure::Dynamic(c)) => { c }
                    Some(FFIProcedure::Native(native)) => {
                        let sp = self.f.len() - 1;
                        self.f[sp].push(native(interns, &self.c, call));

                        return Ok(VM::Running(self));
                    }
                }
            }
            _ => {
                return Err(Error::CallNotCompound);
            }
        };


        let mut f = StackFrame::new_on(c);
        f.push(call);
        self.f.push(f);
        Ok(VM::Running(self))
    }

    pub fn update(mut self, interns: &Interns) -> Runtime<VM<'a>> {
        let sp = self.f.len() - 1;
        let ip = self.f[sp].ip;

        if ip >= self.f[sp].c.instructions.len() { return Err(Error::OutOfCode); }
        self.f[sp].ip += 1;

        use Instruction2::*;
        match self.f[sp].c.instructions[ip] {
            Push(Operand::Integer(i)) => {
                self.f[sp].push(Value::Integer(i));
                Ok(VM::Running(self))
            }

            Push(Operand::Bool(b)) => {
                self.f[sp].push(Value::Bool(b));
                Ok(VM::Running(self))
            }

            Set(vp) => {
                let s1 = self.f[sp].pop()?;
                self.f[sp].v[vp.0] = Some(s1);
                Ok(VM::Running(self))
            }
            SetAssert(vp) => {
                // ZEKKA NOTE: Consider dropping this instruction, replace it with setassert
                let s1 = self.f[sp].pop()?;
                match &self.f[sp].v[vp.0] {
                    None => {
                        self.f[sp].v[vp.0] = Some(s1);
                        Ok(VM::Running(self))
                    }
                    Some(x) => {
                        if x == &s1 { Ok(VM::Running(self)) }
                        else { Err(Error::SetAssertFailed) }
                    }
                }
            }
            Get(vp) => {
                let to_push = match &self.f[sp].v[vp.0] {
                    None => { return Err(Error::GetUnset) }
                    Some(x) => { x.clone() }
                };
                self.f[sp].push(to_push);
                Ok(VM::Running(self))
            }

            Assert => {
                let s1 = self.f[sp].pop()?;
                match s1 {
                    Value::Bool(true) => {},
                    Value::Bool(false) => { return Err(Error::AssertionFailed); }
                    _ => return Err(Error::ConditionalWrongType)
                }
                Ok(VM::Running(self))
            },
            Jump(new_ip) => {
                self.f[sp].ip = new_ip.0;
                Ok(VM::Running(self))
            }
            JumpNo(new_ip) => {
                let s1 = self.f[sp].pop()?;
                match s1 {
                    Value::Bool(true) => {},
                    Value::Bool(false) => {
                        self.f[sp].ip = new_ip.0;
                    }
                    _ => return Err(Error::ConditionalWrongType)
                };
                Ok(VM::Running(self))
            }

            Pop => {
                self.f[sp].pop()?;
                Ok(VM::Running(self))
            }
            Ret => {
                let s1 = self.f[sp].pop()?;
                let top = self.f.pop();

                if self.f.len() == 0 {
                    return Ok(VM::Succeeded(s1, top.unwrap().v));
                }
                self.f[sp - 1].push(s1);
                Ok(VM::Running(self))
            }
            Call => {
                let call = self.f[sp].pop()?;
                self.call(interns, call)
            }

            Mark(mark_ip, keep_on_failure) => {
                let value = self.f[sp].pop()?;

                self.destructure(sp, ip + 1, mark_ip.0, keep_on_failure, value)
            }

            Unmark => {
                Err(Error::UnmarkMustBeMarked)
            }

            DestructCompound(f) => {
                let s1 = self.f[sp].pop()?;
                match s1 {
                    Value::Compound(intern, mut args) if intern == f.0 && args.len() == f.1 => {
                        for arg in args.drain(..).rev() {
                            self.f[sp].push(arg);
                        }
                        Ok(VM::Running(self))
                    }
                    _ => { Err(Error::DestructWrongType) }
                }
            },
            DestructVector(sz) => {
                let s1 = self.f[sp].pop()?;
                match s1 {
                    Value::Vector(mut args) if args.len() == sz => {
                        for arg in args.drain(..).rev() {
                            self.f[sp].push(arg);
                        }
                        Ok(VM::Running(self))
                    }
                    _ => { Err(Error::DestructWrongType) }
                }
            },
            Destruct(sz) => {
                let s1 = self.f[sp].pop()?;
                match s1 {
                    Value::Compound(_, mut args) if args.len() == sz => {
                        for arg in args.drain(..).rev() {
                            self.f[sp].push(arg);
                        }
                        Ok(VM::Running(self))
                    }
                    Value::Vector(mut args) if args.len() == sz => {
                        for arg in args.drain(..).rev() {
                            self.f[sp].push(arg);
                        }
                        Ok(VM::Running(self))
                    }
                    // A set can't be destructed (because its order is arbitrary)
                    _ => { Err(Error::DestructWrongType) }
                }
            }
            ConstructCompound(f) => {
                let len = self.f[sp].s.len();
                if len < f.1 { return Err(Error::NoMoreValues); }
                let values = self.f[sp].s.drain((len - f.1)..len).collect::<Vec<Value>>();
                self.f[sp].push(Value::Compound(f.0, values));
                Ok(VM::Running(self))
            },
            ConstructVector(sz) => {
                let len = self.f[sp].s.len();
                if len < sz { return Err(Error::NoMoreValues); }
                let values = self.f[sp].s.drain((len - sz)..len).collect::<Vec<Value>>();
                self.f[sp].push(Value::Vector(values));
                Ok(VM::Running(self))
            }
            ConstructSet(sz) => {
                let len = self.f[sp].s.len();
                if len < sz { return Err(Error::NoMoreValues); }
                let values = BTreeSet::from_iter(self.f[sp].s.drain((len - sz)..len));
                self.f[sp].push(Value::Set(values));
                Ok(VM::Running(self))
            }

            Equals => {
                let s1 = self.f[sp].pop()?;
                let s2 = self.f[sp].pop()?;
                self.f[sp].push(Value::Bool(s1 == s2));
                Ok(VM::Running(self))
            }

            EqualsOperandAssert(Operand::Bool(b)) => {
                let s1 = self.f[sp].pop()?;
                match s1 {
                    Value::Bool(b2) if b == b2 => { }
                    // TODO: Better error
                    _ => { return Err(Error::AssertionFailed) }
                }
                Ok(VM::Running(self))
            }

            EqualsOperandAssert(Operand::Integer(i)) => {
                let s1 = self.f[sp].pop()?;
                match s1 {
                    Value::Integer(i2) if i == i2 => { }
                    // TODO: Better error
                    _ => { return Err(Error::AssertionFailed) }
                }
                Ok(VM::Running(self))
            }

            Mul => {
                let top = self.f[sp].pop_num2()?;
                self.f[sp].push(match top {
                    Num2::Integer(i1, i2) => Value::Integer(i1 * i2)
                });
                Ok(VM::Running(self))
            }

            Div => {
                let top = self.f[sp].pop_num2()?;
                self.f[sp].push(match top {
                    Num2::Integer(i1, i2) => Value::Integer(i1 / i2)
                });
                Ok(VM::Running(self))
            }

            Add => {
                let top = self.f[sp].pop_num2()?;
                self.f[sp].push(match top {
                    Num2::Integer(i1, i2) => Value::Integer(i1 + i2)
                });
                Ok(VM::Running(self))
            }

            Subtract => {
                let top = self.f[sp].pop_num2()?;
                self.f[sp].push(match top {
                    Num2::Integer(i1, i2) => Value::Integer(i1 - i2)
                });
                Ok(VM::Running(self))
            }

            Le => {
                let top = self.f[sp].pop_num2()?;
                self.f[sp].push(match top {
                    Num2::Integer(i1, i2) => Value::Bool(i1 <= i2)
                });
                Ok(VM::Running(self))
            }

            Ge => {
                let top = self.f[sp].pop_num2()?;
                self.f[sp].push(match top {
                    Num2::Integer(i1, i2) => Value::Bool(i1 >= i2)
                });
                Ok(VM::Running(self))
            }

            Lt => {
                let top = self.f[sp].pop_num2()?;
                self.f[sp].push(match top {
                    Num2::Integer(i1, i2) => Value::Bool(i1 < i2)
                });
                Ok(VM::Running(self))
            }

            Gt => {
                let top = self.f[sp].pop_num2()?;
                self.f[sp].push(match top {
                    Num2::Integer(i1, i2) => Value::Bool(i1 > i2)
                });
                Ok(VM::Running(self))
            }

            Eq => {
                let top = self.f[sp].pop_num2()?;
                self.f[sp].push(match top {
                    Num2::Integer(i1, i2) => Value::Bool(i1 == i2)
                });
                Ok(VM::Running(self))
            }

            Ne => {
                let top = self.f[sp].pop_num2()?;
                self.f[sp].push(match top {
                    Num2::Integer(i1, i2) => Value::Bool(i1 != i2)
                });
                Ok(VM::Running(self))
            }
        }
    }

    fn destructure(
        mut self,
        sp: usize, ip: usize,
        else_ip: usize, keep_on_failure: bool, value: Value
    ) -> Runtime<VM<'a>> {
        // ZEKKA NOTE: Maybe add support for pushes.

        // ZEKKA NOTE: This can theoretically be shared between invocations
        // using set_len() to avoid cleanup/realloc
        let mut temps: Vec<Option<&Value>> =
            Vec::with_capacity(self.f[sp].v.len());
        for _ in 0..self.f[sp].v.len() { temps.push(None); }
        let mut destructure_stack: Vec<&Value> = Vec::new();
        destructure_stack.push(&value);

        // Find instructions that set (so we can check single-assignment only once)
        let mut seek_ip = ip;
        loop {
            use Instruction2::*;
            if seek_ip > self.f[sp].c.instructions.len() { return Err(Error::OutOfCode); }
            match self.f[sp].c.instructions[seek_ip].clone() {
                Unmark => { break; }
                SetAssert(l) => { temps[l.0] = self.f[sp].v[l.0].as_ref(); }
                Pop => { }
                DestructCompound(_) => { }
                DestructVector(_) => { }
                Destruct(_) => { }
                EqualsOperandAssert(_) => { }
                x => {
                    println!("{:?}", x);
                    return Err(Error::MarkInvalidInstruction);
                }
            }
            seek_ip += 1;
        }

        let nopt = |x| {
            match x {
                Some(x) => Ok(x),
                _ => Err(Error::UnmarkWrongStackSize)
            }
        };

        // Check the *possibility* of destructuring
        let mut destructure_ip = ip;
        loop {
            use Instruction2::*;
            match self.f[sp].c.instructions[destructure_ip].clone() {
                Unmark => { break; }
                SetAssert(l) => {
                    let s1 = nopt(destructure_stack.pop())?;
                    match temps[l.0] {
                        None => temps[l.0] = Some(s1),
                        Some(x) => {
                            if x != s1 { return self.destructure_fail(sp, else_ip, keep_on_failure, value); }
                            // continue
                        }
                    }
                }
                Pop => {
                    nopt(destructure_stack.pop())?;
                }
                DestructCompound(f) => {
                    let s1 = nopt(destructure_stack.pop())?;
                    match s1 {
                        Value::Compound(intern, args) if intern == &f.0 && args.len() == f.1 => {
                            for arg in args.iter().rev() {
                                destructure_stack.push(&arg);
                            }
                        }
                        _ => { return self.destructure_fail(sp, else_ip, keep_on_failure, value); }
                    }
                }
                DestructVector(sz) => {
                    let s1 = nopt(destructure_stack.pop())?;
                    match s1 {
                        Value::Vector(args) if args.len() == sz => {
                            for arg in args.iter().rev() {
                                destructure_stack.push(&arg);
                            }
                        }
                        _ => { return self.destructure_fail(sp, else_ip, keep_on_failure, value) }
                    }
                }
                Destruct(sz) => {
                    let s1 = nopt(destructure_stack.pop())?;
                    match s1 {
                        Value::Compound(_, args) if args.len() == sz => {
                            for arg in args.iter().rev() {
                                destructure_stack.push(&arg);
                            }
                        }
                        Value::Vector(args) if args.len() == sz => {
                            for arg in args.iter().rev() {
                                destructure_stack.push(&arg);
                            }
                        }
                        _ => { return self.destructure_fail(sp, else_ip, keep_on_failure, value) }
                    }
                }
                EqualsOperandAssert(Operand::Bool(b)) => {
                    let s1 = nopt(destructure_stack.pop())?;
                    match s1 {
                        Value::Bool(b2) if &b == b2 => { }
                        _ => { return self.destructure_fail(sp, else_ip, keep_on_failure, value) }
                    }
                }
                EqualsOperandAssert(Operand::Integer(i)) => {
                    let s1 = nopt(destructure_stack.pop())?;
                    match s1 {
                        Value::Integer(i2) if &i == i2 => { }
                        _ => { return self.destructure_fail(sp, else_ip, keep_on_failure, value) }
                    }
                }
                _ => { unreachable!() }
            }
            destructure_ip += 1;
        };

        // Write results
        // All errors will panic here as they succeeded in the previous section
        let mut write_stack: Vec<Value> = vec![value];
        let mut write_ip = ip;
        loop {
            use Instruction2::*;
            match self.f[sp].c.instructions[write_ip].clone() {
                Unmark => {
                    self.f[sp].ip = write_ip + 1;
                    return Ok(VM::Running(self));
                }
                SetAssert(l) => {
                    self.f[sp].v[l.0] = Some(write_stack.pop().unwrap());
                }
                Pop => {
                    write_stack.pop().unwrap();
                }
                DestructCompound(_) => {
                    let s1 = write_stack.pop().unwrap();
                    match s1 {
                        Value::Compound(_, mut args) => {
                            for arg in args.drain(..).rev() {
                                write_stack.push(arg);
                            }
                        }
                        _ => { unreachable!(); }
                    }
                }
                DestructVector(_) => {
                    let s1 = write_stack.pop().unwrap();
                    match s1 {
                        Value::Vector(mut args) => {
                            for arg in args.drain(..).rev() {
                                write_stack.push(arg);
                            }
                        }
                        _ => { unreachable!(); }
                    }
                }
                Destruct(_) => {
                    let s1 = write_stack.pop().unwrap();
                    match s1 {
                        Value::Compound(_, mut args) => {
                            for arg in args.drain(..).rev() {
                                write_stack.push(arg);
                            }
                        }
                        Value::Vector(mut args) => {
                            for arg in args.drain(..).rev() {
                                write_stack.push(arg);
                            }
                        }
                        _ => { unreachable!(); }
                    }
                }
                EqualsOperandAssert(_) => {
                    write_stack.pop().unwrap();
                }
                _ => { unreachable!() }
            }
            write_ip += 1;
        }
    }

    fn destructure_fail(
        mut self,
        sp: usize, else_ip: usize,
        keep_on_failure: bool, value: Value
    ) -> Runtime<VM<'a>> {
        self.f[sp].ip = else_ip;
        if keep_on_failure {
            self.f[sp].push(value);
        }
        Ok(VM::Running(self))
    }
}