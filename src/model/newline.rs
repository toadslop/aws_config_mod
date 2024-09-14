use std::{fmt::Display, ops::Deref};

use nom::{branch::alt, bytes::complete::tag, character::complete::crlf};

use crate::lexer::{Parsable, ParserOutput};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Newline<'a>(&'a str);

impl<'a> Display for Newline<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Deref for Newline<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Parsable<'a> for Newline<'a> {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, nl) = alt((tag("\n"), crlf))(input)?;

        Ok((next, Self(nl)))
    }
}
