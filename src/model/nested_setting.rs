//! Contains items related to parsing and stringifying settings which are nested under other settings.
//! Nested settings are identified by the presence of a setting without a value followed by a list of
//! settings with values which are preceded by at least one space or tab character.

use super::{
    equal::Equal, indent::Indent, setting_name::SettingName, value::Value, whitespace::Whitespace,
};
use crate::lexer::{Parsable, ParserOutput};
use std::fmt::Display;

/// Represents a nested setting in its entirety, including indentation, its name and value, and a comment.
/// Since the AWS config file is not recursive, we have this separate type to represent the nested item
/// to avoid defining an unnecessary recursive type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NestedSetting {
    /// Giving the line `region = us-east-2`, 'region' is the [SettingName]
    pub(crate) setting_name: SettingName,

    /// Giving the line `region = us-east-2`, 'use-east-2' is the [Value]
    pub(crate) value: Value,

    /// Giving the line `region = us-east-2`, ' = ' is the [Equal]. It includes option padding of spaces
    /// before and after the equal sign.
    pub(crate) equal: Equal,

    /// Giving the line `  region = us-east-2`, the leading two spaces are the [Indent].
    pub(crate) leading_spaces: Indent,

    /// Giving the line `  region = us-east-2 # This is a comment`, ' # This is a comment' is the [Whitespace].
    /// Any whitespace before or after the setting is not included here.
    pub(crate) whitespace: Whitespace,
}

impl NestedSetting {
    /// Given the setting `region = us-east-2`, this will return 'region'
    pub fn name(&self) -> &SettingName {
        &self.setting_name
    }

    /// Given the setting `region = us-east-2`, this will return 'us-east-2'
    pub fn value(&self) -> &Value {
        &self.value
    }
}

impl Display for NestedSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.leading_spaces, self.setting_name, self.equal, self.value, self.whitespace
        )
    }
}

impl<'a> Parsable<'a> for NestedSetting {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, leading_spaces) = Indent::parse(input)?;
        let (next, setting_name) = SettingName::parse(next)?;
        let (next, equal) = Equal::parse(next)?;
        let (next, value) = Value::parse(next)?;
        let (next, whitespace) = Whitespace::parse(next)?;
        let setting = Self {
            setting_name,
            value,
            equal,
            leading_spaces,
            whitespace,
        };

        Ok((next, setting))
    }
}

#[cfg(test)]
mod test {
    use super::NestedSetting;
    use crate::lexer::Parsable;

    #[test]
    fn parses_a_setting_no_spaces() {
        let setting = "region=us-west-2\n";

        let (rest, set) = NestedSetting::parse(setting).expect("Should be valid");

        assert!(rest.is_empty());

        assert_eq!(*"us-west-2", *set.value);

        assert_eq!(set.to_string(), setting)
    }
}
