pub type Runtime<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UpdatedWhileUpdating, // happens if an update runs into ownership issues
    OutOfCode, // happens if we run to the end of the procedure (because I don't handle that yet)
    NoMoreFrames, // happens if we run out of stack frames, probably because a return is missed
    NoMoreValues, // happens if the stack runs out of expressions. (code-gen error)

    SetAssertFailed, // when the SetAssert instruction fails
    AssertionFailed, // for assertion, false
    GetUnset, // for Get -- when called on an unset variable

    ConditionalWrongType, // for conditional, wrong type (not a bool)
    DestructWrongType, // when attempting to Destruct the wrong type
    NotNumbers, // for numeric operation, both tops must be numbers

    NoSuchProcedure, // for calls to a nonexistent procedure
    CallNotCompound, // a procedure name is a functor, so only compounds are callable

    CantMarkTwice, // for code that tries to mark more than once at the same time on the same frame
    UnmarkMustBeMarked, // for code that unmarks while not marked
    UnmarkWrongStackSize, // for code that unmarks with the wrong number of stack items (not the same number as when marked)
    UnwindMustBeMarked, // for code that unwinds while not marked
    UnwindStackTooSmall, // for code that unwinds when there are less stack elements than it started with
    MarkInvalidInstruction, // for marked code that tries to use an instruction that is not allowed
}
