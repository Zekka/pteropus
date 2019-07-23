use crate::bump::Bump;
use crate::bump::collections::Vec as BVec;
use crate::interns::{Intern, Interns};
use crate::irs::executable1::{FFIProcedure, Executable1};
use crate::primitive::{Functor, Value};
use crate::satc::Satc;
use crate::vm::{BValue, SBV};

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
                box move |bump, interns, module, sbv| _print(ok, bump, interns, module, sbv)
            )
        );
    }
}

fn _print<'bump>(ok: Intern, bump: &'bump Bump, interns: &Interns, executable: &Executable1, value: SBV<'bump>) -> SBV<'bump> {
    match value.as_immut() {
        BValue::Compound(_, v) if v.len() == 1 => {
            _really_print(interns, executable, &v[0]);
            print!("\n");
            io::stdout().flush().unwrap();
        }
        _ => unreachable!(),
    }
    Satc::new(bump.alloc(BValue::Compound(ok, BVec::new())))
}

// TODO: Take interns from an external source too.
fn _really_print(interns: &Interns, executable: &Executable1, value: &BValue) {
    match value {
        BValue::Bool(tf) => print!("{}", tf),
        BValue::Compound(x, xs) => {
            match interns.to_string(*x) {
                None => print!("#{}", x.raw()),
                Some(s) => print!("{}", s),
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
        BValue::Integer(i) => print!("{}", i),
        BValue::Set(xs) => {
            print!("s[");
            for (i, x) in xs.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                _really_print(interns, executable, x);
            }
            print!("]");
        }
        BValue::Vector(xs) => {
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