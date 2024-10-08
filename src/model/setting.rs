//! Contains items related to parsing and stringifying settings. Settings are found under section
//! headers and are comprised of a settign name followed by and equals sign, which is then followed by
//! one of the following:
//!
//! - a value, which is a sequence of non-whitespace characters excluding the # character
//! - a newline followed by a list of indented settings

use super::{
    equal::Equal, setting_name::SettingName, value_type::ValueType, whitespace::Whitespace,
};
use crate::lexer::{Parsable, ParserOutput};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

/// Represents a setting in its entirety, including indentation, its name and value, and a comment
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Setting {
    /// The whitespace and comments on the preceding line
    pub(crate) leading_whitespace: Whitespace,

    /// The name of the [Setting]
    pub(crate) setting_name: SettingName,

    /// The value of the setting. A top-level setting such as this may contain [crate::NestedSetting]s, so
    /// the [ValueType] of this field indicates whether this is the case or not.
    pub(crate) value: ValueType,

    /// The '=' sign appearing between the value and the name. This is to track what the original formatting
    /// of the setting was before it was parsed so that it can be returned to its original state, even after editing.
    pub(crate) equal: Equal,
}

impl Setting {
    /// Create a new [Setting]
    pub fn new(setting_name: SettingName, value: ValueType) -> Self {
        Self {
            leading_whitespace: Whitespace::newline(),
            setting_name,
            value,
            equal: Equal::default(),
        }
    }

    /// Retrieve an immutable reference to the [SettingName]
    pub fn name(&self) -> &SettingName {
        &self.setting_name
    }

    /// Retrieve an immutable reference to the [ValueType]
    pub fn value(&self) -> &ValueType {
        &self.value
    }
}

impl Display for Setting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.leading_whitespace, self.setting_name, self.equal, self.value,
        )
    }
}

impl<'a> Parsable<'a> for Setting {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, leading_whitespace) = Whitespace::parse(input)?;
        let (next, setting_name) = SettingName::parse(next)?;
        let (next, equal) = Equal::parse(next)?;
        let (next, value) = ValueType::parse(next)?;

        let setting = Self {
            setting_name,
            value,
            equal,
            leading_whitespace,
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
        let setting = r#"
region=us-west-2"#;

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
  endpoint_url = https://profile-b-ec2-endpoint.aws
"#;

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

    #[test]
    fn parses_a_nested_setting_with_topline_comment() {
        let setting = r#"ec2 = # some comment here
  endpoint_url = https://profile-b-ec2-endpoint.aws
"#;

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

    #[test]
    fn parses_a_nested_setting_with_nested_line_comment() {
        let setting = r#"ec2 = 
  endpoint_url = https://profile-b-ec2-endpoint.aws # some comment here
"#;

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

    #[test]
    fn parses_a_nested_setting_trailing_comment() {
        let setting = r#"ec2 = 
  endpoint_url = https://profile-b-ec2-endpoint.aws
  # some comment here
"#;

        let (_, set) = Setting::parse(setting).expect("Should be valid");

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
