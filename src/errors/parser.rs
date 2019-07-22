use nom::{
    error::{VerboseError},
};

pub type Error<'a> = VerboseError<&'a str>;
