//! Items related to how '=' signs are parsed and stringified. This type is internal and
//! should not be exposed directly to end users.

use crate::lexer::{equal, Parsable, ParserOutput};
use nom::{character::complete::space0, combinator::recognize, Parser};
use std::{fmt::Display, ops::Deref};

/// Represents an equal sign and it's surrounding whitespace.
/// This is an internal type used mainly to help return a file to its original state,
/// even preserving unusual space.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Equal(String);

impl Equal {
    /// Generate a new [Equal] instance padded on the left and right by one space.
    pub fn padded(padding: usize) -> Self {
        let front_pad = " ".repeat(padding);
        let back_pad = " ".repeat(padding);
        let inner = format!("{}={}", front_pad, back_pad);

        Self(inner)
    }
}

impl Default for Equal {
    fn default() -> Self {
        Self::padded(1)
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
        recognize(space0.and(equal).and(space0))
            .map(str::to_string)
            .map(Self)
            .parse(input)
    }
}
