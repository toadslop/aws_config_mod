use super::{SectionName, SectionType, SettingName};
use crate::lexer::Parsable;
use nom::{
    combinator::{eof, opt},
    error::VerboseError,
};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigPath<'a> {
    pub(crate) section_type: SectionType<'a>,
    pub(crate) section_name: Option<SectionName<'a>>,
    pub(crate) setting_name: SettingName<'a>,
    pub(crate) nested_setting_name: Option<SettingName<'a>>,
}

impl<'a> TryFrom<&'a str> for ConfigPath<'a> {
    type Error = ConfigPathError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (_, config_path) = ConfigPath::parse(value).map_err(to_owned_input)?;

        Ok(config_path)
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

impl<'a> Parsable<'a> for ConfigPath<'a> {
    type Output = Self;

    fn parse(input: &'a str) -> crate::lexer::ParserOutput<'a, Self::Output> {
        let (next, section_type) = SectionType::parse(input)?;
        let (next, section_name) = if let SectionType::Default = section_type {
            (next, None)
        } else {
            let (next, setting_name) = SectionName::parse(next)?;

            (next, Some(setting_name))
        };

        let (next, setting_name) = SettingName::parse(next)?;
        let (next, nested_setting_name) = opt(SettingName::parse)(next)?;
        let (next, _) = eof(next)?;

        let config_path = Self {
            section_type,
            section_name,
            setting_name,
            nested_setting_name,
        };

        Ok((next, config_path))
    }
}
