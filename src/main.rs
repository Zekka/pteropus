#![allow(dead_code)]
#![feature(bind_by_move_pattern_guards)]
#![feature(box_patterns)]
#![feature(box_syntax)]

#[macro_use]
extern crate nom;

mod ast;
mod bytecode;
mod compiler;
mod dbgdump;
mod executable;
mod library;
mod parser;
mod prim;
mod repl;
mod vm;

fn main() {
    let parsed = parser::parse_module(r##"
    fn main {
        if let scott(@alpha, beta) = scott(alpha, delta) {

        }
        eval call print(got(past, first, conditional)).

        let @alpha = beta.
        let @no = no.
        if let yes = @no {
            ret 0.
        }
        else if let yes(bro) = yes(brah) {
            ret 1.
        }
        else if let yes(@dude) = yes(dude) {
            if let @dude = bro {
                ret 2.
            } 
            else if let @dude = dude {
                ret r(3, @dude).
            }
            ret 4.
        } 
        else {
            ret 5.
        }
    }
    "##); // should return r(3, dude)!!!!

    let compiled = parsed.unwrap().compile(library::Standard);

    let ready_to_run = compiled.unwrap();

    println!("Code: {}", ready_to_run.dump());
    repl::repl_main(&ready_to_run);

    /*
    let mut runtime = vm::VM::go(
        &ready_to_run, 
        vm::Value::Compound(ready_to_run.to_intern("main").unwrap(), vec![])
    ).unwrap();

    while runtime.is_running() {
        runtime.update();
    }

    println!("Final state: {:?}", runtime);
    */
}
