use crate::lexer::{Parsable, ParserOutput};
use nom::{bytes::complete::tag, character::complete::space0};
use std::{fmt::Display, ops::Deref};

/// Represents an equal sign and it's surrounding whitespace.
/// This is an internal type used mainly to help return a file to its original state,
/// even preserving unusual space.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Equal<'a>(&'a str);

impl<'a> Default for Equal<'a> {
    fn default() -> Self {
        Self(" = ")
    }
}

impl<'a> Display for Equal<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Deref for Equal<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Parsable<'a> for Equal<'a> {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, leading_ws) = space0(input)?;
        let (next, eq) = tag("=")(next)?;
        let (next, trailing_ws) = space0(next)?;
        let equal = &input[0..(leading_ws.len() + eq.len() + trailing_ws.len())];
        Ok((next, Self(equal)))
    }
}
