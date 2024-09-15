use super::{
    equal::Equal, indent::Indent, setting_name::SettingName, value::Value, whitespace::Whitespace,
};
use crate::lexer::{Parsable, ParserOutput};
use std::{borrow::Cow, fmt::Display};

/// Represents a nested setting in its entirety, including indentation, its name and value, and a comment.
/// Since the AWS config file is not recursive, we have this separate type to represent the nested item
/// to avoid defining an unnecessary recursive type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NestedSetting<'a> {
    pub(crate) setting_name: SettingName<Cow<'a, str>>,
    pub(crate) value: Value<'a>,
    pub(crate) equal: Equal<'a>,
    pub(crate) leading_spaces: Indent<'a>,
    pub(crate) whitespace: Whitespace<'a>,
}

impl<'a> NestedSetting<'a> {
    pub fn name(&self) -> &SettingName<Cow<'a, str>> {
        &self.setting_name
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}

impl<'a> Display for NestedSetting<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.leading_spaces, self.setting_name, self.equal, self.value, self.whitespace
        )
    }
}

impl<'a> Parsable<'a> for NestedSetting<'a> {
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
        let setting = "region=us-west-2";

        let (rest, set) = NestedSetting::parse(setting).expect("Should be valid");

        assert!(rest.is_empty());

        assert_eq!(*"us-west-2", *set.value);

        assert_eq!(set.to_string(), setting)
    }
}
