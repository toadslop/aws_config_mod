use crate::lexer::{Parsable, ParserOutput};
use nom::{branch::alt, bytes::complete::tag, character::complete::alphanumeric1, combinator::map};
use std::{borrow::Cow, fmt::Display};

/// Represents the various section types of an AWS config file. If an unknown section type is
/// encountered, rather than failing it's value is collected under [SectionType::Other]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum SectionType<'a> {
    #[default]
    Default,
    Profile,
    SsoSession,
    Services,
    Plugins,
    Preview,
    Other(Cow<'a, str>),
}

impl<'a> PartialEq<str> for SectionType<'a> {
    fn eq(&self, other: &str) -> bool {
        match self {
            SectionType::Default => Self::DEFAULT == other,
            SectionType::Profile => Self::PROFILE == other,
            SectionType::SsoSession => Self::SSO_SESSION == other,
            SectionType::Services => Self::SERVICES == other,
            SectionType::Plugins => Self::PLUGINS == other,
            SectionType::Preview => Self::PREVIEW == other,
            SectionType::Other(o) => o == other,
        }
    }
}

impl<'a> PartialEq<SectionType<'a>> for str {
    fn eq(&self, other: &SectionType<'a>) -> bool {
        match other {
            SectionType::Default => SectionType::DEFAULT == self,
            SectionType::Profile => SectionType::PROFILE == self,
            SectionType::SsoSession => SectionType::SSO_SESSION == self,
            SectionType::Services => SectionType::SERVICES == self,
            SectionType::Plugins => SectionType::PLUGINS == self,
            SectionType::Preview => SectionType::PREVIEW == self,
            SectionType::Other(other) => other == self,
        }
    }
}

impl<'a> SectionType<'a> {
    const PROFILE: &'static str = "profile";
    const DEFAULT: &'static str = "default";
    const SSO_SESSION: &'static str = "sso-session";
    const SERVICES: &'static str = "services";
    const PLUGINS: &'static str = "plugins";
    const PREVIEW: &'static str = "preview";
}

impl<'a> Display for SectionType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_str = match self {
            SectionType::Profile => Self::PROFILE,
            SectionType::Default => Self::DEFAULT,
            SectionType::SsoSession => Self::SSO_SESSION,
            SectionType::Services => Self::SERVICES,
            SectionType::Plugins => Self::PLUGINS,
            SectionType::Preview => Self::PREVIEW,
            SectionType::Other(other) => other,
        };

        write!(f, "{as_str}")
    }
}

impl<'a> Parsable<'a> for SectionType<'a> {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        alt((
            map(tag(Self::PROFILE), |_| Self::Profile),
            map(tag(Self::DEFAULT), |_| Self::Default),
            map(tag(Self::SSO_SESSION), |_| Self::SsoSession),
            map(tag(Self::SERVICES), |_| Self::Services),
            map(tag(Self::PLUGINS), |_| Self::Plugins),
            map(tag(Self::PREVIEW), |_| Self::Preview),
            map(alphanumeric1, |other| Self::Other(Cow::Borrowed(other))),
        ))(input)
    }
}
