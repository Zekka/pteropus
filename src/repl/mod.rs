use crate::executable;
use crate::parser::parse_repl_statement;

use std::collections::HashMap;
use std::io;
use std::io::Write;

pub fn repl_main(loaded: &executable::Module) {
    let mut scope = HashMap::new();

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).unwrap();

        let parsed = parse_repl_statement(&inp).unwrap();
        let (vars, code) = parsed.compile_repl(loaded).unwrap();
        // println!("Vars, code: {:?}", (&vars, &code));

        let mut start = crate::vm::Running {
            code: loaded,
            frames: vec![crate::vm::StackFrame::new_on(&code)],
        };
        for (k, v) in vars.iter() {
            start.frames[0].vars[*v] = scope.remove(k);
        }

        let mut vm = crate::vm::VM::Running(start);

        while vm.is_running() {
            vm.update();
        }

        match vm {
            crate::vm::VM::Succeeded(_, mut vmvars) => {
                for (k, v) in vars.iter() {
                    let mut extracted: Option<crate::vm::Value> = None;
                    std::mem::swap(&mut extracted, &mut vmvars[*v]);
                    if let Some(value) = extracted {
                        scope.insert(k.to_owned(), value);
                    }
                }
            }
            crate::vm::VM::Failed(e) => {
                println!("Failed: {:?}", e);
            }
            _ => {}
        }



        // println!("You wrote: {:?}", compiled);
    }
}