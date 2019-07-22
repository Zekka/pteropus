use crate::interns::Interns;
use crate::irs::executable1::Executable1;
use crate::parser::parse_repl_statement;
use crate::primitive::Value;
use crate::vm::VM;

use std::collections::HashMap;
use std::io;
use std::io::Write;

pub fn repl_main(base_interns: &Interns, loaded: &Executable1) {
    let mut scope = HashMap::new();
    let mut interns = base_interns.extend();

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).unwrap();

        let parsed = parse_repl_statement(&inp).unwrap();
        let (vars, code) = parsed.compile_repl(&mut interns).unwrap();
        // println!("Vars, code: {:?}", (&vars, &code));

        let mut vm = VM::start_repl(
            &code, &loaded,
            &vars, &mut scope,
        );

        while vm.is_running() {
            vm.update(&interns);
        }

        match vm {
            crate::vm::VM::Succeeded(_, mut vmvars) => {
                for (k, v) in vars.iter() {
                    let mut extracted: Option<Value> = None;
                    std::mem::swap(&mut extracted, &mut vmvars[v.0]);
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