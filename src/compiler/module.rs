use crate::ast;
use crate::executable;
use crate::library::Library;

use std::collections::HashMap;

use super::errors::*;
use super::phases::PreInterns;

impl ast::Module {
    pub fn compile(self, library: impl Library) -> Compiler<executable::Module> {
        let mut preinterns = PreInterns::new();
        let mut procedures = HashMap::new();
        library.add_functions(&mut preinterns, &mut procedures);
        /*
        for (functor, function) in library.functions() {
            procedures.insert(functor, function);
        }
        */

        for procedure in self.procedures {
            let compiled = procedure.compile(&mut preinterns)?;
            // TODO: Catch multiply-defined procedures
            procedures.insert(compiled.functor, executable::FFIProcedure::Dynamic(compiled));
        }

        Ok(executable::Module {
            interns: preinterns.compile(),
            procedures: procedures,
        })
    }
}