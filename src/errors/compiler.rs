pub type Compiler<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    AlreadyAnchored(usize),
    NotAnchored(usize),

    DuplicatedArg(String),
}