mod runner;
mod stackframe;

use crate::errors::runtime::*;
use crate::interns::Interns;
use crate::irs::executable1::Executable1;
use crate::irs::procedure2::Procedure2;
use crate::primitive::Local;
use crate::primitive::Value;

use std::collections::HashMap;
use std::mem;

use runner::Runner;
use stackframe::StackFrame;


#[derive(Debug)]
pub enum VM<'a> {
    Updating,
    Running(Runner<'a>),
    Succeeded(Value, Vec<Option<Value>>), // keep the vars from the frame, to extract in the repl
    Failed(Error),
}

impl<'a> VM<'a> {
    pub fn start_repl(
        repl_proc: &'a Procedure2,
        executable: &'a Executable1,
        var_alloc: &HashMap<String, Local>,
        var_value: &mut HashMap<String, Value>,
    ) -> Self {
        let mut runner = Runner {
            c: executable,
            f: vec![crate::vm::StackFrame::new_on(&repl_proc)],
        };
        for (k, v) in var_alloc.iter() {
            runner.f[0].v[v.0] = var_value.remove(k);
        } ;
        VM::Running(runner)
    }

    pub fn go<'proto>(interns: &Interns<'proto>, code: &'a Executable1, call: Value) -> Runtime<Self> {
        Runner::new(code).call(interns, call)
    }

    pub fn update<'proto>(&mut self, interns: &Interns<'proto>) {
        let mut tar = VM::Updating;
        mem::swap(self, &mut tar);

        *self = match tar {
            VM::Updating => { VM::Failed(Error::UpdatedWhileUpdating) }
            VM::Running(running) => {
                match running.update(interns) {
                    Err(e) => { VM::Failed(e) }
                    Ok(v) => { v }
                }
            }
            VM::Succeeded(succeeded, vars) => { VM::Succeeded(succeeded, vars) }
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

