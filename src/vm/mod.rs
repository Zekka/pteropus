mod bvalue;
mod runner;
mod stackframe;

use crate::bump::Bump;
use crate::errors::runtime::*;
use crate::interns::Interns;
use crate::irs::executable1::Executable1;
use crate::irs::procedure2::Procedure2;
use crate::primitive::Local;
use crate::primitive::Value;
use crate::satc::Satc;
use crate::vm::bvalue::*;

use std::collections::HashMap;
use std::mem;

use runner::Runner;
use stackframe::StackFrame;

pub use bvalue::BValue;
pub use bvalue::SBV;

#[derive(Debug)]
pub enum VM<'bump, 'code> {
    Updating,
    Running(Runner<'bump, 'code>),
    Succeeded(Value, Vec<Option<Value>>), // keep the vars from the frame, to extract in the repl
    Failed(Error),
}

impl<'bump, 'code> VM<'bump, 'code> {
    pub fn start_repl(
        bump: &'bump Bump,
        repl_proc: &'code Procedure2,
        executable: &'code Executable1,
        var_alloc: &HashMap<String, Local>,
        var_value: &mut HashMap<String, Value>,
    ) -> Self {
        let mut runner = Runner {
            c: executable,
            f: vec![crate::vm::StackFrame::new_on(&repl_proc)],
        };
        for (k, v) in var_alloc.iter() {
            let val = var_value.get(k);
            if let Some(val) = val {
                runner.f[0].v[v.0] = Some(Satc::new(bump.alloc(BValue::lower(bump, &val))));
            }
        } ;
        VM::Running(runner)
    }

    pub fn go<'proto>(
        bump: &'bump Bump,
        interns: &Interns<'proto>,
        code: &'code Executable1,
        call: &Value
    ) -> Runtime<Self> {
        let runner: Runner<'bump, 'code> = Runner::new(code);
        let callv: BValue<'bump> = BValue::lower(bump, call);
        runner.call(bump, interns, Satc::new_borrowed(bump.alloc(callv)))
    }

    pub fn update<'proto>(&mut self, bump: &'bump Bump, interns: &Interns<'proto>) {
        let mut tar = VM::Updating;
        mem::swap(self, &mut tar);

        *self = match tar {
            VM::Updating => { VM::Failed(Error::UpdatedWhileUpdating) }
            VM::Running(running) => {
                match running.update(bump, interns) {
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

