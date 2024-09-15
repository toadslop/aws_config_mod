use super::{SectionName, SectionType};
use crate::lexer::Parsable;
use nom::{bytes::complete::tag, combinator::eof, error::VerboseError};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionPath<'a> {
    pub(crate) section_type: SectionType<'a>,
    pub(crate) section_name: Option<SectionName<'a>>,
}

impl<'a> TryFrom<&'a str> for SectionPath<'a> {
    type Error = ConfigPathError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (_, section_path) = Self::parse(value).map_err(to_owned_input)?;

        Ok(section_path)
    }
}

impl<'a> TryFrom<(&'a str, &'a str)> for SectionPath<'a> {
    type Error = ConfigPathError;

    fn try_from((section_type, section_name): (&'a str, &'a str)) -> Result<Self, Self::Error> {
        let (next, section_type) = SectionType::parse(section_type).map_err(to_owned_input)?;
        eof(next).map_err(to_owned_input)?;
        let (next, section_name) = SectionName::parse(section_name).map_err(to_owned_input)?;
        eof(next).map_err(to_owned_input)?;

        let config_path = (section_type, Some(section_name)).into();
        Ok(config_path)
    }
}

impl<'a> TryFrom<(SectionType<'a>, &'a str)> for SectionPath<'a> {
    type Error = ConfigPathError;

    fn try_from(
        (section_type, section_name): (SectionType<'a>, &'a str),
    ) -> Result<Self, Self::Error> {
        let (next, section_name) = SectionName::parse(section_name).map_err(to_owned_input)?;
        eof(next).map_err(to_owned_input)?;

        let config_path = (section_type, Some(section_name)).into();
        Ok(config_path)
    }
}

impl<'a> From<(SectionType<'a>, Option<SectionName<'a>>)> for SectionPath<'a> {
    fn from((section_type, section_name): (SectionType<'a>, Option<SectionName<'a>>)) -> Self {
        SectionPath {
            section_type,
            section_name,
        }
    }
}

fn to_owned_input(error: nom::Err<VerboseError<&str>>) -> nom::Err<VerboseError<String>> {
    error.map(|error| VerboseError {
        errors: error
            .errors
            .into_iter()
            .map(|(a, b)| (a.to_string(), b))
            .collect::<Vec<_>>(),
    })
}

#[derive(Debug, Error)]
pub enum ConfigPathError {
    #[error("Failed to parse config path from string:\n\t{0}")]
    ParseError(#[from] nom::Err<VerboseError<String>>),
}

impl<'a> Parsable<'a> for SectionPath<'a> {
    type Output = Self;

    fn parse(input: &'a str) -> crate::lexer::ParserOutput<'a, Self::Output> {
        let (next, section_type) = SectionType::parse(input)?;
        let (next, _) = tag(".")(next)?;
        let (next, section_name) = if let SectionType::Default = section_type {
            (next, None)
        } else {
            let (next, setting_name) = SectionName::parse(next)?;

            (next, Some(setting_name))
        };

        let config_path = Self {
            section_type,
            section_name,
        };

        Ok((next, config_path))
    }
}
