//! Handles parsing, reading, and updating values of aws credntials files.

use super::SectionName;
use super::{header::CredentialHeader, whitespace::Whitespace, Section};
use crate::lexer::{to_owned_input, Parsable};
use nom::Parser;
use nom::{combinator::eof, multi::many0, sequence::tuple};
use std::str::FromStr;

/// Represents and aws credentials file. A credentials file contains sensitive authentication information
/// separately from the main configuration file.
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
    /// Initialize an empty credentials file
    pub fn new() -> Self {
        Self {
            leading_whitespace: Whitespace::default(),
            profiles: vec![],
            trailing_whitespace: Whitespace::newline(),
        }
    }

    // TODO: rename 'get section'. Add a strongly typed struct for CredentialProfile and use 'get_profile' for that
    /// Get an immutable reference to the credentials for a given profile
    pub fn get_profile(&self, profile_name: SectionName) -> Option<&Section<CredentialHeader>> {
        self.profiles
            .iter()
            .find(|profile| *profile.get_name() == profile_name)
    }

    /// Get a mutable reference to the credentials for a given profile
    pub fn get_profile_mut(
        &mut self,
        profile_name: SectionName,
    ) -> Option<&mut Section<CredentialHeader>> {
        self.profiles
            .iter_mut()
            .find(|profile| *profile.get_name() == profile_name)
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
