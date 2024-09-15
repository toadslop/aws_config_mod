use super::{
    header::Header, nested_setting::NestedSetting, SectionName, SectionType, Setting, SettingName,
};
use crate::lexer::{Parsable, ParserOutput};
use nom::multi::many0;
use std::fmt::Display;

/// Represents an entire section, including the section type, the profile name, and all of the settings
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Section<'a> {
    pub(crate) header: Header<'a>,
    pub(crate) settings: Vec<Setting<'a>>,
}

impl<'a> Section<'a> {
    pub fn get_type(&self) -> &SectionType {
        &self.header.section_type
    }

    pub fn get_name(&self) -> Option<&SectionName> {
        self.header.section_name.as_ref()
    }

    pub fn settings(&self) -> &[Setting<'a>] {
        &self.settings
    }

    pub fn get_setting(&self, setting_name: SettingName) -> Option<&Setting> {
        self.settings
            .iter()
            .find(|setting| *setting.name() == setting_name)
    }

    pub fn get_nested_setting(
        &self,
        setting_name: SettingName,
        nested_setting_name: SettingName,
    ) -> Option<&NestedSetting> {
        let setting = self.get_setting(setting_name)?;

        match setting.value() {
            super::ValueType::Single(_) => None,
            super::ValueType::Nested(nested) => nested
                .iter()
                .find(|setting| *setting.name() == nested_setting_name),
        }
    }
}

impl<'a> Display for Section<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.header,
            self.settings
                .iter()
                .map(Setting::to_string)
                .collect::<String>()
        )
    }
}

impl<'a> Parsable<'a> for Section<'a> {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, header) = Header::parse(input)?;
        let (next, settings) = many0(Setting::parse)(next)?;
        let section = Self { header, settings };

        Ok((next, section))
    }
}

#[cfg(test)]
mod test {
    use super::Section;
    use crate::lexer::Parsable;

    const SAMPLE_SECTION: &str = r#"[default] # This is my comment
region=us-west-2
output=json"#;

    const MULTIPLE_NESTED: &str = r#"[services profileB]
ec2 = 
  endpoint_url = https://profile-b-ec2-endpoint.aws
dynamodb = 
  endpoint_url = http://localhost:8000
"#;

    #[test]
    fn parses_section_with_two_entries_not_nested() {
        let (rest, section) = Section::parse(SAMPLE_SECTION).expect("Should be valid");
        assert!(rest.is_empty());
        let settings = section.settings;
        let first = &settings[0];

        assert_eq!(**first.name(), "region");
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
        let (rest, section) = Section::parse(MULTIPLE_NESTED).expect("Should be valid");
        assert!(rest.is_empty());
        let settings = &section.settings;
        let first = &settings[0];

        assert_eq!(**first.name(), "ec2");
        match first.value() {
            crate::ValueType::Single(_) => panic!("Should not be single"),
            crate::ValueType::Nested(nested) => nested,
        };

        let second = &settings[1];
        assert_eq!(**second.name(), "dynamodb");
        match second.value() {
            crate::ValueType::Single(_) => panic!("Should not be single"),
            crate::ValueType::Nested(nested) => nested,
        };

        assert_eq!(&section.to_string(), MULTIPLE_NESTED)
    }
}
