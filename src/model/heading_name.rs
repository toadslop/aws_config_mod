use crate::lexer::{Parsable, ParserOutput};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::alphanumeric1, combinator::recognize,
    multi::many1_count,
};
use std::{fmt::Display, ops::Deref};

/// Represents the custom profile name associated with a section. In other words, if we see
/// [profile dev], then 'dev' is the profile name
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default, Hash)]
pub struct HeadingName<'a>(&'a str);

impl<'a> Display for HeadingName<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Deref for HeadingName<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Parsable<'a> for HeadingName<'a> {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, heading_name) =
            recognize(many1_count(alt((alphanumeric1, tag("_"), tag("-")))))(input)?;

        Ok((next, Self(heading_name)))
    }
}
