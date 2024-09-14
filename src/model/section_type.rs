use crate::lexer::{Parsable, ParserOutput};
use nom::{branch::alt, bytes::complete::tag, combinator::map};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum SectionType {
    #[default]
    Default,
    Profile,
    SsoSession,
    Services,
}

impl Display for SectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_str = match self {
            SectionType::Profile => "profile",
            SectionType::Default => "default",
            SectionType::SsoSession => "sso-session",
            SectionType::Services => "services",
        };

        write!(f, "{as_str}")
    }
}

impl<'a> Parsable<'a> for SectionType {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        alt((
            map(tag("profile"), |_| Self::Profile),
            map(tag("default"), |_| Self::Default),
            map(tag("sso-session"), |_| Self::SsoSession),
            map(tag("services"), |_| Self::Services),
        ))(input)
    }
}
