#[derive(Debug)]
pub struct Module {
    pub procedures: Vec<Procedure>,
}

#[derive(Debug)]
pub struct Block(pub Vec<Statement>);

#[derive(Debug)]
pub struct Procedure {
    pub name: String,
    pub args: Vec<Pattern>,
    pub body: Block,
}

#[derive(Debug)]
pub enum Statement {
    If(Condition, Block, Option<Block>),
    Assign(String, Expression),
    Destructure(Pattern, Expression),
    Eval(Expression),
    Ret(Expression),
}

#[derive(Clone, Copy, Debug)]
pub enum BinOp {
    And, Or, 

    Multiply, Divide,
    Add, Subtract,

    Le, Ge,
    Lt, Gt,
    Eq, Ne,
}

#[derive(Eq, PartialEq)]
pub enum Associativity {
    Lhs, Rhs
}

#[derive(Debug)]
pub enum Pattern {
    IntLiteral(i64), // TODO: Allow negation as a special case
    Variable(String),
    Compound(String, Vec<Pattern>),
    WcCompound(Vec<Pattern>), // wildcard head 
    Vector(Vec<Pattern>),
}

#[derive(Debug)]
pub enum Condition {
    Let(Pattern, Expression), 
    Bare(Expression),
}

#[derive(Debug)]
pub enum Expression {
    NoOp, // Use the thing that's already on top of the stack. FFI

    IntLiteral(i64),
    Variable(String),
    Call(Box<Expression>),
    Compound(String, Vec<Expression>),
    Vector(Vec<Expression>),
    Set(Vec<Expression>),

    Binary(Box<Expression>, BinOp, Box<Expression>)
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
    pub fn precedence(self) -> (u8, Associativity) {
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

    pub fn tighter(self, other: BinOp) -> bool {
        let (prec_1, assoc_1) = self.precedence();
        let (prec_2, _assoc_2) = other.precedence();
        // it's assumed (and unchecked) that if prec_1 == prec_2, assoc_1 == assoc_2

        if prec_1 < prec_2 { return true }
        if prec_1 == prec_2 && assoc_1 == Associativity::Lhs { return true }
        return false;
    }
}