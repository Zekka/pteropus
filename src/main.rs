#![allow(dead_code)]
#![feature(bind_by_move_pattern_guards)]
#![feature(box_patterns)]
#![feature(box_syntax)]

#[macro_use]
extern crate nom;

mod bump;
mod bumpclone;
mod compiler;
mod errors;
mod interns;
mod irs;
mod library;
mod parser;
mod primitive;
mod repl;
mod satc;
mod vm;

pub fn main() {
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
                eval call print(got(right, answer)).
                ret r(3, @dude).
            }
            ret 4.
        }
        else {
            ret 5.
        }
    }
    "##); // should return r(3, dude)!!!!

    let mut interns = interns::Interns::new(0);
    let compiled = parsed.unwrap().compile(&mut interns, library::Standard);

    let ready_to_run = compiled.unwrap();

    // println!("Code: {}", ready_to_run.dump());
    repl::repl_main(&interns, &ready_to_run);
}