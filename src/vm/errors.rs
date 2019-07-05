pub type Runtime<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UpdatedWhileUpdating, // happens if an update runs into ownership issues
    OutOfCode, // happens if we run to the end of the procedure (because I don't handle that yet)
    NoMoreFrames, // happens if we run out of stack frames, probably because a return is missed
    NoMoreValues, // happens if the stack runs out of expressions. (code-gen error)

    AssertionFailed, // for assertion, false
    GetUnset, // for Get -- when called on an unset variable

    ConditionalWrongType, // for conditional, wrong type (not a bool)
    NotNumbers, // for numeric operation, both tops must be numbers

    NoSuchProcedure, // for calls to a nonexistent procedure
    CallNotCompound, // a procedure name is a functor, so only compounds are callable
}
