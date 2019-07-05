use crate::ast;
use crate::executable;

use std::collections::HashMap;

use super::errors::*;
use super::phases::PreInterns;

impl ast::Module {
    pub fn compile(self) -> Compiler<executable::Module> {
        let mut preinterns = PreInterns::new();
        let mut procedures = HashMap::new();

        for procedure in self.procedures {
            let compiled = procedure.compile(&mut preinterns)?;
            // TODO: Catch multiply-defined procedures
            procedures.insert(compiled.functor, compiled);
        }

        Ok(executable::Module {
            interns: preinterns.compile(),
            procedures: procedures,
        })
    }
}