#[derive(Debug)]
pub struct Module {
    pub procedures: Vec<Procedure>,
}

#[derive(Debug)]
pub struct Procedure {
    pub name: String,
    pub args: Vec<Pattern>,
    pub body: Block,
}

#[derive(Debug)]
pub struct Block(pub Vec<Statement>);

#[derive(Debug)]
pub enum Statement {
    If(Condition, Block, Option<Block>),
    Assign(String, Expression),
    Destructure(Pattern, Expression),
    Eval(Expression),
    Ret(Expression),
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

#[derive(Clone, Copy, Debug)]
pub enum BinOp {
    And, Or,

    Multiply, Divide,
    Add, Subtract,

    Le, Ge,
    Lt, Gt,
    Eq, Ne,
}
