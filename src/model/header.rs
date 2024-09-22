//! Items related to parsing and stringifying headers from aws config files. A header
//! is enclosed in square brackets [] and my contain one to two identifiers. For example,
//! [default] and [profile A] are both valid headers

use super::{
    section_name::SectionName, section_type::SectionType, whitespace::Whitespace, SectionPath,
};
use crate::lexer::{Parsable, ParserOutput};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{delimited, separated_pair},
};
use std::fmt::Display;

/// A header of a config section. Contains the section type as well as the profile.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct ConfigHeader {
    /// The name of the section. For example, in the heading [profile A], 'A' is the name.
    pub(crate) section_name: Option<SectionName>,

    /// The section type. For example, in [profile A], 'profile' is the section type.
    pub(crate) section_type: SectionType,

    /// Any whitespace or comment which follows the header
    pub(crate) whitespace: Whitespace,
}

impl ConfigHeader {
    /// Provided a [SectionType] and optional [SectionName], creates a new section [Header].
    pub fn new(section_type: SectionType, section_name: Option<SectionName>) -> Self {
        Self {
            section_name,
            section_type,
            whitespace: Default::default(),
        }
    }

    /// Indicates whether this header belongs to the default profile
    pub fn is_default_profile(&self) -> bool {
        self.section_name
            .as_ref()
            .map(|inner| *inner == *"default")
            .unwrap_or_default()
            && self.section_type == SectionType::Profile
    }
}

impl From<SectionPath> for ConfigHeader {
    fn from(value: SectionPath) -> Self {
        Self::new(value.section_type, value.section_name)
    }
}

impl Display for ConfigHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(section_name) = &self.section_name {
            if self.is_default_profile() {
                write!(f, "[{}]{}", section_name, self.whitespace)
            } else {
                write!(
                    f,
                    "[{} {}]{}",
                    self.section_type, section_name, self.whitespace
                )
            }
        } else {
            write!(f, "[{}]{}", self.section_type, self.whitespace)
        }
    }
}

impl<'a> Parsable<'a> for ConfigHeader {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, (section_name, section_type)) = delimited(
            tag("["),
            alt((
                map(tag("default"), |default: &str| {
                    (Some(SectionName(default.to_string())), SectionType::Profile)
                }),
                map(
                    separated_pair(SectionType::parse, tag(" "), SectionName::parse),
                    |(section_type, section_name)| (Some(section_name), section_type),
                ),
                map(SectionType::parse, |section_type| (None, section_type)),
            )),
            tag("]"),
        )(input)?;

        let (next, whitespace) = Whitespace::parse(next)?;

        let header = Self {
            section_name,
            section_type,
            whitespace,
        };

        Ok((next, header))
    }
}

/// Represents the header of a [crate::Section] in a credentials file. In a credentials file,
/// headers never have a second value -- they just contain a profile name and are implicitly of
/// [crate::SectionType::Profile].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct CredentialHeader {
    /// The name of the profile associated with this section. In a credential file,
    /// all sections are of type [SectionType::Profile], so only the profile name
    /// is recorded in the header as an instance of [SectionName]. For example,
    /// if in the main config file we have [profile A], then the corresponding
    /// entry in the credentials file is just [A]
    pub(crate) profile_name: SectionName,

    /// Any whitespace or comment which follows the header
    pub(crate) whitespace: Whitespace,
}

impl CredentialHeader {
    /// Crate a new [CredentialHeader] with the provided
    pub fn new(profile_name: SectionName) -> Self {
        Self {
            profile_name,
            whitespace: Whitespace::default(),
        }
    }

    /// A credential header is always of [SectionType::Profile], so this function always returns that.
    pub fn get_type(&self) -> &SectionType {
        &SectionType::Profile
    }

    /// Get the name the profile that this section of the credentials file is associated with.
    pub fn get_name(&self) -> &SectionName {
        &self.profile_name
    }
}

impl Display for CredentialHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.profile_name)
    }
}

impl<'a> Parsable<'a> for CredentialHeader {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, profile_name) = delimited(tag("["), SectionName::parse, tag("]"))(input)?;

        let (next, whitespace) = Whitespace::parse(next)?;

        let header = Self {
            profile_name,
            whitespace,
        };

        Ok((next, header))
    }
}
