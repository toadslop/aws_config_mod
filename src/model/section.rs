//! Contains items related to parsing and stringifying entire sections, including the header and all the settings.

use super::{
    header::{ConfigHeader, CredentialHeader},
    nested_setting::NestedSetting,
    whitespace::Whitespace,
    SectionName, SectionType, Setting, SettingName, Value, ValueType,
};
use crate::lexer::{Parsable, ParserOutput};
use nom::multi::many0;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

/// Represents an entire section, including the section type, the profile name, and all of the settings
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Section<T> {
    /// Any whitespace or comments that appear before the section
    pub(crate) leading_whitespace: Whitespace,

    /// The section header, which contains the optional section type and the section name
    pub(crate) header: T,

    /// The list of settings for the section
    pub(crate) settings: Vec<Setting>,

    /// Any whitespace or comments that appear after the section
    pub(crate) trailing_whitespace: Whitespace,
}

impl<T> Section<T>
where
    T: Default,
{
    /// Create a new section, without any settings.
    pub fn new(header: T) -> Self {
        Self {
            header,
            ..Default::default()
        }
    }

    /// Retrieve the [SectionType] of this [Section]
    pub fn get_type(&self) -> &SectionType {
        todo!()
        // &self.header.section_type
    }

    /// Get an immutable reference to the [Setting]s of this section
    pub fn settings(&self) -> &[Setting] {
        &self.settings
    }

    /// Retrieve a specific [Setting] by [SettingName], if it exists
    pub fn get_setting(&self, setting_name: &SettingName) -> Option<&Setting> {
        self.settings
            .iter()
            .find(|setting| setting.name() == setting_name)
    }

    /// Get an immutable reference to the value pointed to by a given [SettingName],
    /// if it exists. Note that because values can be nested, this returns an enum,
    /// [ValueType] which contains a single value or the list of nested values.
    pub fn get_value(&self, setting_name: &SettingName) -> Option<&ValueType> {
        self.get_setting(setting_name)
            .map(|setting| setting.value())
    }

    /// Update the value attached to a given setting name.
    pub fn set_value(&mut self, setting_name: &SettingName, value: ValueType) {
        if let Some(setting) = self.get_setting_mut(setting_name) {
            setting.value = value
        }
    }

    /// Retrieve a mutable reference to a [Setting] by its [SettingName], if it exists
    pub fn get_setting_mut(&mut self, setting_name: &SettingName) -> Option<&mut Setting> {
        self.settings
            .iter_mut()
            .find(|setting| setting.name() == setting_name)
    }

    /// Lookup a [NestedSetting], which is a setting nested under another setting, if it exists.
    pub fn get_nested_setting(
        &self,
        setting_name: &SettingName,
        nested_setting_name: &SettingName,
    ) -> Option<&NestedSetting> {
        let setting = self.get_setting(setting_name)?;

        match setting.value() {
            super::ValueType::Single(_) => None,
            super::ValueType::Nested(nested) => nested
                .iter()
                .find(|setting| setting.name() == nested_setting_name),
        }
    }

    /// Change the [Value] of an existing [Setting]
    pub fn set(&mut self, setting_name: SettingName, value: Value) {
        if let Some(setting) = self.get_setting_mut(&setting_name) {
            setting.value = ValueType::Single(value);
        } else {
            let value = ValueType::Single(value);
            let setting = Setting::new(setting_name, value);
            self.settings.push(setting)
        }
    }
}

impl Section<ConfigHeader> {
    /// Get the optional [SectionName] of this section if it exists
    pub fn get_name(&self) -> Option<&SectionName> {
        self.header.section_name.as_ref()
    }
}

impl Section<CredentialHeader> {
    /// Get the optional [SectionName] of this section
    pub fn get_name(&self) -> &SectionName {
        &self.header.profile_name
    }
}

impl<T> Display for Section<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.leading_whitespace,
            self.header,
            self.settings
                .iter()
                .map(Setting::to_string)
                .collect::<String>(),
            self.trailing_whitespace,
        )
    }
}

impl<'a, T> Parsable<'a> for Section<T>
where
    T: Parsable<'a, Output = T>,
{
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, leading_whitespace) = Whitespace::parse(input)?;
        let (next, header) = T::parse(next)?;
        let (next, settings) = many0(Setting::parse)(next)?;
        let (next, trailing_whitespace) = Whitespace::parse(next)?;
        let section = Self {
            header,
            settings,
            leading_whitespace,
            trailing_whitespace,
        };

        Ok((next, section))
    }
}

#[cfg(test)]
mod test {
    use super::Section;
    use crate::{lexer::Parsable, model::header::ConfigHeader};

    const SAMPLE_SECTION: &str = r#"[default] # This is my comment
region=us-west-2
output=json"#;

    const LEADING_COMMENT: &str = r#"# This is my comment
[default]
region=us-west-2
output=json"#;

    const MULTIPLE_LEADING_COMMENT: &str = r#"# This is my comment
# This is my comment
[default]
region=us-west-2
output=json"#;

    const MULTIPLE_TRAILING_COMMENT: &str = r#"[default]
region=us-west-2
output=json
# This is my comment
# This is my comment
"#;

    const MULTIPLE_NESTED: &str = r#"[services profileB]
ec2 = 
  endpoint_url = https://profile-b-ec2-endpoint.aws
dynamodb = 
  endpoint_url = http://localhost:8000
"#;

    #[test]
    fn parses_section_with_two_entries_not_nested() {
        let (rest, section) =
            Section::<ConfigHeader>::parse(SAMPLE_SECTION).expect("Should be valid");
        assert!(rest.is_empty());
        let settings = section.settings;
        let first = &settings[0];

        assert_eq!(**first.name(), *"region");
        match first.value() {
            crate::ValueType::Single(_) => (),
            crate::ValueType::Nested(_) => panic!("Should not be nested"),
        }

        let second = &settings[1];
        match second.value() {
            crate::ValueType::Single(_) => (),
            crate::ValueType::Nested(_) => panic!("Should not be nested"),
        }
    }

    #[test]
    fn multiple_nested() {
        let (rest, section) =
            Section::<ConfigHeader>::parse(MULTIPLE_NESTED).expect("Should be valid");
        assert!(rest.is_empty());
        let settings = &section.settings;

        let first = &settings[0];

        assert_eq!(**first.name(), *"ec2");
        match first.value() {
            crate::ValueType::Single(_) => panic!("Should not be single"),
            crate::ValueType::Nested(nested) => nested,
        };

        let second = &settings[1];
        assert_eq!(**second.name(), *"dynamodb");
        match second.value() {
            crate::ValueType::Single(_) => panic!("Should not be single"),
            crate::ValueType::Nested(nested) => nested,
        };

        assert_eq!(&section.to_string(), MULTIPLE_NESTED)
    }

    #[test]
    fn leading_comment_on_section() {
        let (_, section) =
            Section::<ConfigHeader>::parse(LEADING_COMMENT).expect("Should be valid");

        assert_eq!(&section.to_string(), LEADING_COMMENT)
    }

    #[test]
    fn multiple_leading_comment_on_section() {
        let (_, section) =
            Section::<ConfigHeader>::parse(MULTIPLE_LEADING_COMMENT).expect("Should be valid");

        assert_eq!(&section.to_string(), MULTIPLE_LEADING_COMMENT)
    }

    #[test]
    fn multiple_trailing_comment_on_section() {
        let (_, section) =
            Section::<ConfigHeader>::parse(MULTIPLE_TRAILING_COMMENT).expect("Should be valid");

        assert_eq!(&section.to_string(), MULTIPLE_TRAILING_COMMENT)
    }
}
