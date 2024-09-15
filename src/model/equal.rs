use crate::lexer::{Parsable, ParserOutput};
use nom::{bytes::complete::tag, character::complete::space0};
use std::{fmt::Display, ops::Deref};

/// Represents an equal sign and it's surrounding whitespace.
/// This is an internal type used mainly to help return a file to its original state,
/// even preserving unusual space.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Equal(String);

impl Default for Equal {
    fn default() -> Self {
        Self(" = ".to_string())
    }
}

impl Display for Equal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Equal {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Parsable<'a> for Equal {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, leading_ws) = space0(input)?;
        let (next, eq) = tag("=")(next)?;
        let (next, trailing_ws) = space0(next)?;
        let equal = &input[0..(leading_ws.len() + eq.len() + trailing_ws.len())];
        Ok((next, Self(equal.to_string())))
    }
}
