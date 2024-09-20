use super::{header::CredentialHeader, whitespace::Whitespace, Section};
use super::{SectionName, SectionPath};
use crate::lexer::{to_owned_input, Parsable};
use nom::Parser;
use nom::{combinator::eof, multi::many0, sequence::tuple};
use std::str::FromStr;

/// TODO
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AwsCredentialsFile {
    /// Whitespace and comments at the head of the file, before the first section
    pub(crate) leading_whitespace: Whitespace,

    /// Represents the content of the file. The content includes the sections representing
    /// the authentication credentials of specific profiles.
    pub(crate) profiles: Vec<Section<CredentialHeader>>,

    /// Whitespace and comments at the end of the file, after the end of the last section
    pub(crate) trailing_whitespace: Whitespace,
}

impl AwsCredentialsFile {
    /// TODO:
    pub fn new() -> Self {
        Self {
            leading_whitespace: Whitespace::default(),
            profiles: vec![],
            trailing_whitespace: Whitespace::newline(),
        }
    }

    /// TODO
    pub fn get_profile(&self, profile_name: SectionPath) -> &Section<CredentialHeader> {
        self.profiles
            .iter()
            .find(|profile| *profile.get_name().0 == profile_name.section_name);

        todo!()
    }
}

impl<'a> Parsable<'a> for AwsCredentialsFile {
    type Output = Self;

    fn parse(input: &'a str) -> crate::lexer::ParserOutput<'a, Self::Output> {
        let (next, ((leading_whitespace, profiles, trailing_whitespace), _)) = tuple((
            Whitespace::parse,
            many0(Section::<CredentialHeader>::parse),
            Whitespace::parse,
        ))
        .and(eof)
        .parse(input)?;

        let config_file = Self {
            leading_whitespace,
            profiles,
            trailing_whitespace,
        };

        Ok((next, config_file))
    }
}

impl FromStr for AwsCredentialsFile {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
            .map(|a| a.1)
            .map_err(to_owned_input)
            .map_err(crate::Error::from)
    }
}
