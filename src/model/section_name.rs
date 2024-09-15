use crate::lexer::{Parsable, ParserOutput};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::alphanumeric1, combinator::recognize,
    multi::many1_count,
};
use std::{borrow::Cow, fmt::Display, ops::Deref};

/// Represents the custom profile name associated with a section. In other words, if we see
/// [profile dev], then 'dev' is the profile name
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default, Hash)]
pub struct SectionName<'a>(Cow<'a, str>);

impl<'a> PartialEq<str> for SectionName<'a> {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl<'a> PartialEq<SectionName<'a>> for str {
    fn eq(&self, other: &SectionName) -> bool {
        self == other.0
    }
}

impl<'a> Display for SectionName<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Deref for SectionName<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Parsable<'a> for SectionName<'a> {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, heading_name) =
            recognize(many1_count(alt((alphanumeric1, tag("_"), tag("-")))))(input)?;

        Ok((next, Self(Cow::Borrowed(heading_name))))
    }
}
