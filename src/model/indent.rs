use crate::lexer::{Parsable, ParserOutput};
use nom::character::complete::multispace0;
use std::{fmt::Display, ops::Deref};

/// Represents non-comment whitespace at the head of start of a line
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
pub(crate) struct Indent(String);

impl Display for Indent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Indent {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Parsable<'a> for Indent {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, ws) = multispace0(input)?;

        Ok((next, Self(ws.to_string())))
    }
}
