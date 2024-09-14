use super::{equal::Equal, setting::Setting, setting_name::SettingName, whitespace::Whitespace};
use crate::lexer::{Parsable, ParserOutput};
use nom::multi::many0;
use std::fmt::Display;

/// Represents a setting that has nested sub-settings.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct SubsectionSetting<'a> {
    pub setting_name: SettingName<'a>,
    equal: Equal<'a>,
    pub whitespace: Whitespace<'a>,
    pub sub_settings: Vec<Setting<'a>>,
}

impl<'a> Display for SubsectionSetting<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.setting_name,
            self.equal,
            self.whitespace,
            self.sub_settings
                .iter()
                .map(Setting::to_string)
                .collect::<String>()
        )
    }
}

impl<'a> Parsable<'a> for SubsectionSetting<'a> {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, setting_name) = SettingName::parse(input)?;
        let (next, equal) = Equal::parse(next)?;
        let (next, whitespace) = Whitespace::parse(next)?;
        let (next, sub_settings) = many0(Setting::parse)(next)?;
        let sub_section = Self {
            setting_name,
            equal,
            whitespace,
            sub_settings,
        };

        Ok((next, sub_section))
    }
}
