use super::{setting::Setting, subsection_setting::SubsectionSetting};
use crate::lexer::{Parsable, ParserOutput};
use std::fmt::Display;

/// Represents an entry in a config section. It could be a single-line setting
/// or a setting with subsections
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Entry<'a> {
    /// A single-line setting
    Single(Setting<'a>),

    /// A setting with nested sub-settings
    WithSubsettings(SubsectionSetting<'a>),
}

impl<'a> Display for Entry<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entry::Single(setting) => write!(f, "{setting}"),
            Entry::WithSubsettings(subsection) => write!(f, "{subsection}"),
        }
    }
}

impl<'a> Parsable<'a> for Entry<'a> {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        if let Ok((next, setting)) = Setting::parse(input) {
            return Ok((next, Self::Single(setting)));
        };

        let (next, subsection_setting) = SubsectionSetting::parse(input)?;

        Ok((next, Self::WithSubsettings(subsection_setting)))
    }
}

#[cfg(test)]
mod test {
    use crate::{lexer::Parsable, model::entry::Entry};

    #[test]
    fn parses_a_setting_no_spaces() {
        let setting = "region=us-west-2";

        let (rest, setting) = Entry::parse(setting).expect("Should be valid");

        assert!(rest.is_empty());

        match setting {
            Entry::Single(setting) => assert_eq!("us-west-2", *setting.value),
            Entry::WithSubsettings(_) => panic!("Should be top-level"),
        }
    }
}
