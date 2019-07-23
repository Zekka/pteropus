use crate::interns::{Intern, Interns};
use crate::irs::executable1::{FFIProcedure, Executable1};
use crate::primitive::{Functor, Value};

use std::collections::HashMap;
use std::io;
use std::io::Write;

pub trait Library {
    fn add_functions(
        &self,
        interns: &mut Interns,
        procedures: &mut HashMap<Functor<Intern>, FFIProcedure>,
    );
}


pub struct Standard;

impl Library for Standard {
    fn add_functions(
        &self,
        interns: &mut Interns,
        procedures: &mut HashMap<Functor<Intern>, FFIProcedure>,
    ) {
        let ok = interns.intern("ok");
        procedures.insert(
            Functor(interns.intern("print"), 1), FFIProcedure::Native(
                box move |interns, module, value| _print(ok, interns, module, value)
            )
        );
    }
}

fn _print(ok: Intern, interns: &Interns, executable: &Executable1, value: Value) -> Value {
    match &value {
        Value::Compound(_, v) if v.len() == 1 => {
            _really_print(interns, executable, &v[0]);
            print!("\n");
            io::stdout().flush().unwrap();
        }
        _ => unreachable!(),
    }
    Value::Compound(ok, vec![])
}

// TODO: Take interns from an external source too.
fn _really_print(interns: &Interns, executable: &Executable1, value: &Value) {
    match value {
        Value::Bool(tf) => print!("{}", tf),
        Value::Compound(x, xs) => {
            match interns.to_string(*x) {
                None => print!("#{}", x.raw()),
                Some(s) => print!(":{}", s),
            }
            if xs.len() > 0 {
                print!("(");
                for (i, x) in xs.iter().enumerate() {
                    if i > 0 {
                        print!(", ");
                    }
                    _really_print(interns, executable, x);
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
                _really_print(interns, executable, x);
            }
            print!("]");
        }
        Value::Vector(xs) => {
            print!("v[");
            for (i, x) in xs.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                _really_print(interns, executable, x);
            }
            print!("]");
        }
    }
}