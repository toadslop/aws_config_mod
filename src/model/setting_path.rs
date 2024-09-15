use super::{section_path::ConfigPathError, SectionPath, SettingName};
use crate::{lexer::Parsable, util::to_owned_input};
use nom::{bytes::complete::tag, combinator::eof};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingPath {
    pub(crate) section_path: SectionPath,
    pub(crate) setting_name: SettingName,
}

impl TryFrom<&str> for SettingPath {
    type Error = ConfigPathError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (_, config_path) = Self::parse(value).map_err(to_owned_input)?;

        Ok(config_path.to_owned())
    }
}

impl TryFrom<(&str, &str, &str)> for SettingPath {
    type Error = ConfigPathError;

    fn try_from(
        (section_type, section_name, setting_name): (&str, &str, &str),
    ) -> Result<Self, Self::Error> {
        let section_path = SectionPath::try_from((section_type, section_name))?;
        let (_, setting_name) = SettingName::parse(setting_name).map_err(to_owned_input)?;

        Ok(Self {
            section_path,
            setting_name,
        })
    }
}

impl<'a> Parsable<'a> for SettingPath {
    type Output = Self;

    fn parse(input: &'a str) -> crate::lexer::ParserOutput<'a, Self::Output> {
        let (next, section_path) = SectionPath::parse(input)?;
        let (next, _) = tag(".")(next)?;
        let (next, setting_name) = SettingName::parse(next)?;
        let (next, _) = eof(next)?;

        let config_path = Self {
            section_path,
            setting_name,
        };

        Ok((next, config_path))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NestedSettingPath {
    pub(crate) section_path: SectionPath,
    pub(crate) setting_name: SettingName,
    pub(crate) nested_setting_name: SettingName,
}

impl TryFrom<&str> for NestedSettingPath {
    type Error = ConfigPathError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (_, nested_path) = Self::parse(value).map_err(to_owned_input)?;

        Ok(nested_path)
    }
}

impl<'a> Parsable<'a> for NestedSettingPath {
    type Output = Self;

    fn parse(input: &'a str) -> crate::lexer::ParserOutput<'a, Self::Output> {
        let (next, setting_path) = SettingPath::parse(input)?;
        let (next, nested_setting_name) = SettingName::parse(next)?;
        eof(next)?;

        let nested_path = Self {
            section_path: setting_path.section_path,
            setting_name: setting_path.setting_name,
            nested_setting_name,
        };

        Ok((next, nested_path))
    }
}
