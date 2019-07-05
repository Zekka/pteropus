use crate::bytecode::RawInstruction;
use crate::executable::{Module, Procedure};

impl Module {
    pub fn dump(&self) -> String {
        // TODO: Use the trait version of join()
        let mut all = self.procedures.iter().collect::<Vec<_>>();
        all.sort_by(|(k1, _), (k2, _)| k1.partial_cmp(k2).unwrap());
        all.iter().map(|(_, v)| v.dump()).collect::<Vec<String>>().join("\n\n")
    }
}

impl Procedure {
    pub fn dump(&self) -> String {
        // TODO: Use the trait version of join()
        format!(
            "fn {}/{} {{\n{}\n}}", 
            self.functor.0,
            self.functor.1,
            self.instructions.iter().map(|i| i.dump()).collect::<Vec<String>>().join("\n"),
        )
    }
}

impl RawInstruction {
    fn dump(&self) -> String {
        format!("  {:?}", self)
    }
}

// impl RawInstruction {
//     fn dump(&self) -> String {
//         match self {
//             Instruction::Push(StackItem::Label(lbl)) => { format!("  push :{}", lbl) }
//             Instruction::Push(StackItem::Local(loc)) => { format!("  push @{}", loc) }
//             Instruction::Push(StackItem::MetaNumber(mn)) => { format!("  push '{:?}", mn) }
//             Instruction::Push(StackItem::Operand(Operand::Bool(b))) => { format!("  push {:?}", b) }
//             Instruction::Push(StackItem::Operand(Operand::Integer(i))) => { format!("  push {:?}", i) }
//             Instruction::Push(StackItem::Operand(Operand::String(s))) => { format!("  push {:?}", s) }
//             Instruction::Word(w) => { w.mnemonic() }
//         }
//     }
// }
// 
// impl Word {
//     fn mnemonic(&self) -> String {
//         format!("  {:?}", self).to_lowercase()
//     }
// }