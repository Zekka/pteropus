use crate::interns::Interns;
use crate::irs::ast1;
use crate::irs::executable1;
use crate::library::Library;

use std::collections::HashMap;

use crate::errors::compiler::*;

use executable1::{Executable1, FFIProcedure};

impl ast1::Module {
    pub fn compile(self, interns: &mut Interns, library: impl Library) -> Compiler<Executable1> {
        let mut procedures = HashMap::new();

        library.add_functions(interns, &mut procedures);

        for procedure in self.procedures {
            let compiled = procedure.compile(interns)?;
            procedures.insert(compiled.functor, FFIProcedure::Dynamic(compiled));
        }

        Ok(Executable1 { procedures })
    }
}
