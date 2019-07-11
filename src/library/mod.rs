use crate::compiler::phases::PreInterns;
use crate::executable::{FFIProcedure, Module};
use crate::prim::Functor;
use crate::vm::Value;

use std::collections::HashMap;
use std::io;
use std::io::Write;

pub trait Library {
    fn add_functions(
        &self,
        preinterns: &mut PreInterns,
        procedures: &mut HashMap<Functor<usize>, FFIProcedure>,
    );
}


pub struct Standard;

impl Library for Standard {
    fn add_functions(
        &self,
        preinterns: &mut PreInterns,
        procedures: &mut HashMap<Functor<usize>, FFIProcedure>,
    ) {
        let ok = preinterns.to_intern("ok").0;
        procedures.insert(
            Functor(preinterns.to_intern("print").0, 1), FFIProcedure::Native(box move |module, value| _print(ok, module, value))
        );
    }
}

fn _print(ok: usize, module: &Module, value: Value) -> Value {
    match &value {
        Value::Compound(_, v) if v.len() == 1 => {
            _really_print(module, &v[0]);
            print!("\n");
            io::stdout().flush().unwrap();
        }
        _ => unreachable!(),
    }
    Value::Compound(ok, vec![])
}

// TODO: Take interns from an external source too.
fn _really_print(module: &Module, value: &Value) {
    match value {
        Value::Bool(tf) => print!("{}", tf),
        Value::Compound(x, xs) => {
            match module.from_intern(*x) {
                None => print!("#{}", x),
                Some(s) => print!("{}", s),
            }
            if xs.len() > 0 {
                print!("(");
                for (i, x) in xs.iter().enumerate() {
                    if i > 0 {
                        print!(", ");
                    }
                    _really_print(module, x);
                }
                print!(")");
            }
        }
        Value::Integer(i) => print!("{}", i),
        Value::Set(xs) => {
            print!("s[");
            for (i, x) in xs.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                _really_print(module, x);
            }
            print!("]");
        }
        Value::Vector(xs) => {
            print!("v[");
            for (i, x) in xs.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                _really_print(module, x);
            }
            print!("]");
        }
    }
}