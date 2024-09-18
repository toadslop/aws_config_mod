use crate::lexer::{Parsable, ParserOutput};
use nom::{branch::alt, bytes::complete::tag, character::complete::alphanumeric1, combinator::map};
use std::fmt::Display;

/// Represents the various section types of an AWS config file. If an unknown section type is
/// encountered, rather than failing it's value is collected under [SectionType::Other]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum SectionType {
    /// The default section type, referring to the default profile
    #[default]
    Default,

    /// A section which contains a profile definition
    Profile,

    /// A section which contains configuratino for sso sessions
    SsoSession,

    /// A section which contains configurations for various aws services
    Services,

    /// A section which contains configuration for aws cli plugins
    Plugins,

    /// A section which contains preview features which are enabled or disabled
    Preview,

    /// A catchall to store any other section types which may appear
    Other(String),
}

impl PartialEq<str> for SectionType {
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

impl PartialEq<SectionType> for str {
    fn eq(&self, other: &SectionType) -> bool {
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

impl SectionType {
    /// The string represenation of the profile section type
    const PROFILE: &'static str = "profile";

    /// The string represenation of the default section type
    const DEFAULT: &'static str = "default";

    /// The string represenation of the sso-session section type
    const SSO_SESSION: &'static str = "sso-session";

    /// The string represenation of the services section type
    const SERVICES: &'static str = "services";

    /// The string represenation of the plugins section type
    const PLUGINS: &'static str = "plugins";

    /// The string represenation of the preview section type
    const PREVIEW: &'static str = "preview";
}

impl Display for SectionType {
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

impl<'a> Parsable<'a> for SectionType {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        alt((
            map(tag(Self::PROFILE), |_| Self::Profile),
            map(tag(Self::DEFAULT), |_| Self::Default),
            map(tag(Self::SSO_SESSION), |_| Self::SsoSession),
            map(tag(Self::SERVICES), |_| Self::Services),
            map(tag(Self::PLUGINS), |_| Self::Plugins),
            map(tag(Self::PREVIEW), |_| Self::Preview),
            map(alphanumeric1, |other: &str| Self::Other(other.to_string())),
        ))(input)
    }
}
