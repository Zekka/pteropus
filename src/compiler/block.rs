use crate::interns::Interns;
use crate::irs::ast1;
use crate::irs::procedure1;

use crate::errors::compiler::*;

use procedure1::Procedure1;

use ast1::Block;

impl Block {
    pub fn compile(self, it: &mut Interns, pp: &mut Procedure1) -> Compiler<()> {
        for st in self.0 {
            st.compile(it, pp)?;
        };
        Ok(())
    }
}
