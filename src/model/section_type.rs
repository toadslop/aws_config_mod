use crate::lexer::{Parsable, ParserOutput};
use nom::{branch::alt, bytes::complete::tag, character::complete::alphanumeric1, combinator::map};
use std::fmt::Display;

/// Represents the various section types of an AWS config file. If an unknown section type is
/// encountered, rather than failing it's value is collected under [SectionType::Other]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum SectionType<'a> {
    #[default]
    Default,
    Profile,
    SsoSession,
    Services,
    Other(&'a str),
}

impl<'a> Display for SectionType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_str = match self {
            SectionType::Profile => "profile",
            SectionType::Default => "default",
            SectionType::SsoSession => "sso-session",
            SectionType::Services => "services",
            SectionType::Other(other) => other,
        };

        write!(f, "{as_str}")
    }
}

impl<'a> Parsable<'a> for SectionType<'a> {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        alt((
            map(tag("profile"), |_| Self::Profile),
            map(tag("default"), |_| Self::Default),
            map(tag("sso-session"), |_| Self::SsoSession),
            map(tag("services"), |_| Self::Services),
            map(alphanumeric1, Self::Other),
        ))(input)
    }
}
