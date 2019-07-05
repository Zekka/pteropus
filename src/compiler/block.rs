use crate::ast::*;

use super::errors::Compiler;
use super::phases::{PreInterns, PreProcedure};

impl Block {
    pub fn compile(self, pp: &mut PreProcedure, it: &mut PreInterns) -> Compiler<()> {
        for st in self.0 {
            st.compile(pp, it)?;
        };
        Ok(())
    }
}