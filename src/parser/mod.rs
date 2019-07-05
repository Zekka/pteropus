use nom::{
    error::{VerboseError},
};

use crate::ast::*;

pub type Error<'a> = VerboseError<&'a str>;

mod combinator;
mod entry_point;
mod expression;
mod pattern;
mod statement;
mod structural;
mod whitespace;
mod word;

pub use combinator::*;
pub use entry_point::*;
pub use expression::*;
pub use pattern::*;
pub use statement::*;
pub use structural::*;
pub use whitespace::*;
pub use word::*;