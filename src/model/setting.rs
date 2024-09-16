use super::{equal::Equal, indent::Indent, setting_name::SettingName, value_type::ValueType};
use crate::lexer::{Parsable, ParserOutput};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

/// Represents a setting in its entirety, including indentation, its name and value, and a comment
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Setting {
    pub(crate) setting_name: SettingName,
    pub(crate) value: ValueType,
    pub(crate) equal: Equal,
    pub(crate) leading_spaces: Indent,
}

impl Setting {
    pub fn new(setting_name: SettingName, value: ValueType) -> Self {
        Self {
            setting_name,
            value,
            equal: Equal::default(),
            leading_spaces: Indent::default(),
        }
    }
    pub fn name(&self) -> &SettingName {
        &self.setting_name
    }

    pub fn value(&self) -> &ValueType {
        &self.value
    }
}

impl Display for Setting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.leading_spaces, self.setting_name, self.equal, self.value,
        )
    }
}

impl<'a> Parsable<'a> for Setting {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, leading_spaces) = Indent::parse(input)?;
        let (next, setting_name) = SettingName::parse(next)?;
        let (next, equal) = Equal::parse(next)?;
        let (next, value) = ValueType::parse(next)?;

        let setting = Self {
            setting_name,
            value,
            equal,
            leading_spaces,
        };

        Ok((next, setting))
    }
}

#[cfg(test)]
mod test {
    use super::Setting;
    use crate::lexer::Parsable;

    #[test]
    fn parses_a_setting_no_spaces() {
        let setting = "region=us-west-2";

        let (rest, set) = Setting::parse(setting).expect("Should be valid");

        assert!(rest.is_empty());

        assert_eq!("region", set.name());

        match set.value() {
            crate::ValueType::Single(value) => assert_eq!(value, "us-west-2"),
            crate::ValueType::Nested(_) => panic!("Should not be nested"),
        }

        assert_eq!(set.to_string(), setting)
    }

    #[test]
    fn parses_a_nested_setting() {
        let setting = r#"ec2 = 
  endpoint_url = https://profile-b-ec2-endpoint.aws"#;

        let (rest, set) = Setting::parse(setting).expect("Should be valid");

        assert!(rest.is_empty());

        assert_eq!("ec2", set.name());

        let nested = match set.value() {
            crate::ValueType::Single(_) => panic!("Should be nested"),
            crate::ValueType::Nested(nested) => nested,
        };

        let first = nested.first().expect("Should have a first");

        assert_eq!(first.name(), "endpoint_url");
        assert_eq!(first.value(), "https://profile-b-ec2-endpoint.aws");

        assert_eq!(set.to_string(), setting)
    }
}
