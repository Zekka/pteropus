use super::types::{BinOp, Expression};

#[derive(Eq, PartialEq)]
enum Associativity {
    Lhs, Rhs
}

impl Expression {
    pub fn integrate(self, bop2: BinOp, rhs2: Expression) -> Expression {
        match self {
            Expression::Binary(lhs1, bop1, box rhs1) if !bop1.tighter(bop2) => {
                Expression::Binary(lhs1, bop1, box rhs1.integrate(bop2, rhs2))
            }
            _ => {
                Expression::Binary(box self, bop2, box rhs2)
            }
        }
    }
}

impl BinOp {
    fn precedence(self) -> (u8, Associativity) {
        // 0 binds tighter than 1, binds tighter than 2, and so on
        match self {
            BinOp::Multiply => (10, Associativity::Lhs),
            BinOp::Divide => (10, Associativity::Lhs),
            BinOp::Add => (15, Associativity::Lhs),
            BinOp::Subtract => (15, Associativity::Lhs),

            BinOp::Le => (20, Associativity::Lhs),
            BinOp::Ge => (20, Associativity::Lhs),
            BinOp::Lt => (20, Associativity::Lhs),
            BinOp::Gt => (20, Associativity::Lhs),
            BinOp::Eq => (20, Associativity::Lhs),
            BinOp::Ne => (20, Associativity::Lhs),

            BinOp::And => (25, Associativity::Lhs),
            BinOp::Or => (25, Associativity::Lhs),
        }
    }

    fn tighter(self, other: BinOp) -> bool {
        let (prec_1, assoc_1) = self.precedence();
        let (prec_2, _assoc_2) = other.precedence();
        // it's assumed (and unchecked) that if prec_1 == prec_2, assoc_1 == assoc_2

        if prec_1 < prec_2 { return true }
        if prec_1 == prec_2 && assoc_1 == Associativity::Lhs { return true }
        return false;
    }
}
