use super::{section_name::SectionName, section_type::SectionType, whitespace::Whitespace};
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
pub struct Header<'a> {
    pub(crate) section_name: Option<SectionName<'a>>,
    pub(crate) section_type: SectionType<'a>,
    pub(crate) comment: Whitespace<'a>,
}

impl<'a> Display for Header<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(profile) = &self.section_name {
            write!(f, "[{} {}]{}", self.section_type, profile, self.comment)
        } else {
            write!(f, "[{}]{}", self.section_type, self.comment)
        }
    }
}

impl<'a> Parsable<'a> for Header<'a> {
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

        let (next, comment) = Whitespace::parse(next)?;

        let header = Self {
            section_name,
            section_type,
            comment,
        };

        Ok((next, header))
    }
}
