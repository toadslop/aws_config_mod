use crate::lexer::{Parsable, ParserOutput};
use nom::character::complete::multispace0;
use std::{fmt::Display, ops::Deref};

/// Represents non-comment whitespace at the head of start of a line
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
pub(crate) struct Indent<'a>(&'a str);

impl<'a> Display for Indent<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Deref for Indent<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Parsable<'a> for Indent<'a> {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, ws) = multispace0(input)?;

        Ok((next, Self(ws)))
    }
}
