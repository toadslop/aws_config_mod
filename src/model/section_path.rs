use super::{SectionName, SectionType};
use crate::{lexer::Parsable, util::to_owned_input};
use nom::{bytes::complete::tag, combinator::eof, error::VerboseError};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionPath {
    pub(crate) section_type: SectionType,
    pub(crate) section_name: Option<SectionName>,
}

impl TryFrom<&str> for SectionPath {
    type Error = ConfigPathError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (_, section_path) = Self::parse(value).map_err(to_owned_input)?;

        Ok(section_path)
    }
}

impl TryFrom<(&str, &str)> for SectionPath {
    type Error = ConfigPathError;

    fn try_from((section_type, section_name): (&str, &str)) -> Result<Self, Self::Error> {
        let (next, section_type) = SectionType::parse(section_type).map_err(to_owned_input)?;
        eof(next).map_err(to_owned_input)?;
        let (next, section_name) = SectionName::parse(section_name).map_err(to_owned_input)?;
        eof(next).map_err(to_owned_input)?;

        let config_path = (section_type, Some(section_name)).into();
        Ok(config_path)
    }
}

impl TryFrom<(SectionType, &str)> for SectionPath {
    type Error = ConfigPathError;

    fn try_from((section_type, section_name): (SectionType, &str)) -> Result<Self, Self::Error> {
        let (next, section_name) = SectionName::parse(section_name).map_err(to_owned_input)?;
        eof(next).map_err(to_owned_input)?;

        let config_path = (section_type, Some(section_name)).into();
        Ok(config_path)
    }
}

impl TryFrom<SectionType> for SectionPath {
    type Error = ConfigPathError;

    fn try_from(section_type: SectionType) -> Result<Self, Self::Error> {
        let section_path = match section_type {
            SectionType::Default | SectionType::Preview | SectionType::Plugins => SectionPath {
                section_type,
                section_name: None,
            },
            _ => Err(ConfigPathError::RequiresSectionName)?,
        };

        Ok(section_path)
    }
}

impl From<(SectionType, Option<SectionName>)> for SectionPath {
    fn from((section_type, section_name): (SectionType, Option<SectionName>)) -> Self {
        SectionPath {
            section_type,
            section_name,
        }
    }
}

impl From<(SectionType, SectionName)> for SectionPath {
    fn from((section_type, section_name): (SectionType, SectionName)) -> Self {
        SectionPath {
            section_type,
            section_name: Some(section_name),
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigPathError {
    #[error("Failed to parse config path from string:\n\t{0}")]
    ParseError(#[from] nom::Err<VerboseError<String>>),
    #[error("The provided section type requires a section name")]
    RequiresSectionName,
}

impl<'a> Parsable<'a> for SectionPath {
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
