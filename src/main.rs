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
mod parser;
mod prim;
mod vm;

fn main() {
    let parsed = parser::parse_module(r##"
    fn main() {
        ret v[call fib(0), call fib(1), call fib(2), call fib(3), call fib(4), call fib(5), call fib(6), call fib(7)].
    }

    fn fib(@x) {
        if @x < 2 {
            ret 1.
        }
        ret call fib(@x - 1) + call fib(@x - 2).
    }
    "##);

    let compiled = parsed.unwrap().compile();

    let ready_to_run = compiled.unwrap();

    let mut runtime = vm::VM::go(
        &ready_to_run, 
        vm::Value::Compound(ready_to_run.to_intern("main").unwrap(), vec![])
    ).unwrap();

    while runtime.is_running() {
        runtime.update();
    }

    println!("Final state: {:?}", runtime);
}
