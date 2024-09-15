use super::{
    equal::Equal, indent::Indent, setting_name::SettingName, value::Value, whitespace::Whitespace,
};
use crate::lexer::{Parsable, ParserOutput};
use std::fmt::Display;

/// Represents a setting in its entirety, including indentation, its name and value, and a comment
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Setting<'a> {
    pub(crate) setting_name: SettingName<'a>,
    pub(crate) value: Value<'a>,
    pub(crate) equal: Equal<'a>,
    pub(crate) leading_spaces: Indent<'a>,
    pub(crate) whitespace: Whitespace<'a>,
}

impl<'a> Setting<'a> {
    pub fn name(&self) -> &SettingName {
        &self.setting_name
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn is_nested(&self) -> bool {
        !self.leading_spaces.is_empty()
    }
}

impl<'a> Display for Setting<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.leading_spaces, self.setting_name, self.equal, self.value, self.whitespace
        )
    }
}

impl<'a> Parsable<'a> for Setting<'a> {
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
    use crate::lexer::Parsable;

    use super::Setting;

    #[test]
    fn parses_a_setting_no_spaces() {
        let setting = "region=us-west-2";

        let (rest, set) = Setting::parse(setting).expect("Should be valid");

        assert!(rest.is_empty());

        assert_eq!("us-west-2", *set.value);

        assert_eq!(set.to_string(), setting)
    }
}
