use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{delimited, separated_pair},
};

use crate::lexer::{Parsable, ParserOutput};

use super::{profile_name::ProfileName, section_type::SectionType, whitespace::Whitespace};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Header<'a> {
    profile: Option<ProfileName<'a>>,
    section_type: SectionType,
    comment: Whitespace<'a>,
}

impl<'a> Display for Header<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(profile) = &self.profile {
            write!(f, "[{} {}]{}", self.section_type, profile, self.comment)
        } else {
            write!(f, "[{}]{}", self.section_type, self.comment)
        }
    }
}

impl<'a> Parsable<'a> for Header<'a> {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, (profile, section_type)) = delimited(
            tag("["),
            alt((
                map(
                    separated_pair(SectionType::parse, tag(" "), ProfileName::parse),
                    |(section_type, profile_name)| (Some(profile_name), section_type),
                ),
                map(SectionType::parse, |section_type| (None, section_type)),
            )),
            tag("]"),
        )(input)?;

        let (next, comment) = Whitespace::parse(next)?;

        let header = Self {
            profile,
            section_type,
            comment,
        };

        Ok((next, header))
    }
}
