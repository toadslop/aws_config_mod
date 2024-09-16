use crate::lexer::{Parsable, ParserOutput};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::alphanumeric1, combinator::recognize,
    multi::many1_count,
};
use std::{fmt::Display, ops::Deref};

/// Represents the custom profile name associated with a section. In other words, if we see
/// [profile dev], then 'dev' is the profile name
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default, Hash)]
pub struct SectionName(String);

impl PartialEq<str> for SectionName {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<SectionName> for str {
    fn eq(&self, other: &SectionName) -> bool {
        self == other.0
    }
}

impl Display for SectionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for SectionName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Parsable<'a> for SectionName {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, heading_name) =
            recognize(many1_count(alt((alphanumeric1, tag("_"), tag("-")))))(input)?;

        Ok((next, Self(heading_name.to_string())))
    }
}
