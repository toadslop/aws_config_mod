//! Contains items related to parsing and stringifying section names. Section names are contained within
//! square brackets at the start of sections. If only one item is contained between the square brackets,
//! that is the section name. If two items are contained within the square brackets, the second item is
//! the section name and the first is the section type.

use crate::lexer::{to_owned_input, Parsable, ParserOutput};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::alphanumeric1, combinator::recognize,
    multi::many1_count,
};
use std::{fmt::Display, ops::Deref, str::FromStr};

/// Represents the custom profile name associated with a section. In other words, if we see
/// [profile dev], then 'dev' is the profile name
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default, Hash)]
pub struct SectionName(pub(crate) String);

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

impl FromStr for SectionName {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
            .map(|a| a.1)
            .map_err(to_owned_input)
            .map_err(crate::Error::from)
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
