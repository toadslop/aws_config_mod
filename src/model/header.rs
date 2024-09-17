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
pub struct Header {
    pub(crate) section_name: Option<SectionName>,
    pub(crate) section_type: SectionType,
    pub(crate) whitespace: Whitespace,
}

impl Header {
    /// Provided a [SectionType] and optional [SectionName], creates a new section [Header].
    pub fn new(section_type: SectionType, section_name: Option<SectionName>) -> Self {
        Self {
            section_name,
            section_type,
            whitespace: Default::default(),
        }
    }
}

impl From<SectionPath> for Header {
    fn from(value: SectionPath) -> Self {
        Self::new(value.section_type, value.section_name)
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(profile) = &self.section_name {
            write!(f, "[{} {}]{}", self.section_type, profile, self.whitespace)
        } else {
            write!(f, "[{}]{}", self.section_type, self.whitespace)
        }
    }
}

impl<'a> Parsable<'a> for Header {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, (section_name, section_type)) = delimited(
            tag("["),
            alt((
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
