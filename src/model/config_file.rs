use super::{
    header::Header, whitespace::Whitespace, NestedSetting, NestedSettingPath, Section, SectionName,
    SectionPath, SectionType, Setting, SettingPath, Value,
};
use crate::lexer::{to_owned_input, Parsable};
use nom::{
    combinator::{eof, opt},
    error::VerboseError,
    multi::many0,
    sequence::tuple,
    IResult, Parser,
};
use std::{fmt::Display, str::FromStr};

/// Represents a complete aws config file, including internal tracking of [Whitespace]
#[derive(Debug, PartialEq, Eq, Clone, Hash, Default)]
pub struct AwsConfigFile {
    /// Whitespace and comments at the head of the file, before the first section
    pub(crate) leading_whitespace: Whitespace,

    /// Represents the content of the file. The content includes the sections of the config
    /// as well as full-line whitespace, which includes comments
    pub(crate) sections: Vec<Section>,

    /// Whitespace and comments at the end of the file, after the end of the last section
    pub(crate) trailing_whitespace: Whitespace,
}

impl FromStr for AwsConfigFile {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
            .map(|a| a.1)
            .map_err(to_owned_input)
            .map_err(crate::Error::from)
    }
}

impl AwsConfigFile {
    /// Return the [AwsConfigFile] to its [String] format. This function simply wraps the [Display] implementation.
    pub fn serialize(&self) -> String {
        self.to_string()
    }

    /// Given a [SectionPath], will find and return the [Section] if it exists; otherwise returns [None].
    pub fn get_section(&self, config_path: &SectionPath) -> Option<&Section> {
        let SectionPath {
            section_type,
            section_name,
        } = config_path;

        self.get_section_inner(section_type, section_name.as_ref())
    }

    /// Given a [SettingPath], locate the given [Setting], if it exists.
    pub fn get_setting(&self, setting_path: &SettingPath) -> Option<&Setting> {
        let SettingPath {
            section_path,
            setting_name,
        } = setting_path;

        let section = self.get_section(section_path)?;

        section.get_setting(setting_name)
    }

    /// Retrieves a [NestedSetting], or a setting contained within another setting, given the [NestedSettingPath]
    /// if it exists.
    pub fn get_nested_setting(&self, setting_path: &NestedSettingPath) -> Option<&NestedSetting> {
        let NestedSettingPath {
            section_path,
            setting_name,
            nested_setting_name,
        } = setting_path;

        let section = self.get_section(section_path)?;
        section.get_nested_setting(setting_name, nested_setting_name)
    }

    /// Provided a [SettingPath] and a [Value], locates the desired [Setting] and changes its [Value].
    /// If the setting doesn't exist, it will be created. If the [Section] that contains the [Setting]
    /// doesn't exist, it will also be created.
    pub fn set(&mut self, setting_path: SettingPath, value: Value) {
        let section = match self.get_section_mut(
            &setting_path.section_path.section_type,
            &setting_path.section_path.section_name,
        ) {
            Some(section) => section,
            None => self.insert_section(&setting_path.section_path),
        };

        section.set(setting_path.setting_name, value);
    }

    /// Get an immutable reference to a [Section] by its [SectionType] and [SectionName]
    fn get_section_inner(
        &self,
        section_type: &SectionType,
        section_name: Option<&SectionName>,
    ) -> Option<&Section> {
        self.sections.iter().find(|section| {
            section.header.section_type == *section_type
                && section.header.section_name.as_ref() == section_name
        })
    }

    /// Get a mutable reference to a [Section]
    pub(crate) fn get_section_mut(
        &mut self,
        section_type: &SectionType,
        section_name: &Option<SectionName>,
    ) -> Option<&mut Section> {
        self.sections.iter_mut().find_map(|section| {
            if section.header.section_type == *section_type
                && section.header.section_name == *section_name
            {
                Some(section)
            } else {
                None
            }
        })
    }

    /// Check if the given [Section] exists from a [SectionPath]
    pub(crate) fn contains_section(&self, section_path: &SectionPath) -> bool {
        self.sections.iter().any(|section| {
            section.get_name() == section_path.section_name.as_ref()
                && *section.get_type() == section_path.section_type
        })
    }

    /// Given a [SectionPath], create the [Section] if it doesn't exist and return a mutable
    /// reference to it.
    pub(crate) fn insert_section(&mut self, section_path: &SectionPath) -> &mut Section {
        if !self.contains_section(section_path) {
            let new_section: Section = Section::new(Header::from(section_path.clone()));
            self.sections.push(new_section);
        }

        #[allow(clippy::unwrap_used)]
        // This cannot fail because we just added the item if it didn't exist
        self.get_section_mut(&section_path.section_type, &section_path.section_name)
            .unwrap()
    }
}

impl Display for AwsConfigFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.leading_whitespace,
            self.sections
                .iter()
                .map(Section::to_string)
                .collect::<String>(),
            self.trailing_whitespace
        )
    }
}

impl<'a> Parsable<'a> for AwsConfigFile {
    type Output = Self;

    fn parse(input: &'a str) -> IResult<&'a str, Self::Output, VerboseError<&'a str>> {
        let (next, ((leading_whitespace, sections, trailing_whitespace), _)) = tuple((
            Whitespace::parse,
            opt(many0(Section::parse)),
            Whitespace::parse,
        ))
        .and(eof)
        .parse(input)?;

        let config_file = Self {
            leading_whitespace,
            sections: sections.unwrap_or_default(),
            trailing_whitespace,
        };

        Ok((next, config_file))
    }
}

#[cfg(test)]
mod test {
    use super::AwsConfigFile;
    use crate::lexer::Parsable;
    use nom::Finish;

    const SAMPLE_CONFIG_FILE: &str = r#"
# I am a leading comment
[default] # This is my comment
region=us-west-2
output=json

[profile user1]
region=us-east-1
output=text

[services my-services]
dynamodb = 
  endpoint_url = http://localhost:8000


"#;

    const SAMPLE_CONFIG_FILE_2: &str = r#"
[profile A]
credential_source = Ec2InstanceMetadata
endpoint_url = https://profile-a-endpoint.aws/

[profile B]
source_profile = A
role_arn = arn:aws:iam::123456789012:role/roleB
services = profileB

[services profileB]
ec2 = 
  endpoint_url = https://profile-b-ec2-endpoint.aws
dynamodb = 
  endpoint_url = http://localhost:8000
"#;

    const EMPTY_CONFIG: &str = r#" "#;

    #[test]
    fn parses_sample_config() {
        let (next, config) = AwsConfigFile::parse(SAMPLE_CONFIG_FILE).expect("Should be valid");
        assert!(next.is_empty());

        let mut sections = config.sections.iter();
        let _ = sections.next().unwrap();
        let _ = sections.next().unwrap();

        // TODO: finish this test

        let as_string = config.to_string();
        assert_eq!(as_string, SAMPLE_CONFIG_FILE)
    }

    #[test]
    fn parses_sample_config2() {
        let (next, config) = AwsConfigFile::parse(SAMPLE_CONFIG_FILE_2).expect("Should be valid");
        assert!(next.is_empty());

        let mut sections = config.sections.iter();

        let _ = sections.next().unwrap();

        let _ = sections.next().unwrap();

        // TODO: finish this test

        let as_string = config.to_string();
        assert_eq!(as_string, SAMPLE_CONFIG_FILE_2)
    }

    #[test]
    fn empty_config_should_pass() {
        let (next, _) = AwsConfigFile::parse(EMPTY_CONFIG).expect("Should be valid");

        assert!(next.is_empty())
    }

    #[test]
    fn empty_string_is_valid_config() {
        AwsConfigFile::parse("").finish().ok().unwrap();
    }

    #[test]
    fn multiline_whitespace_is_valid() {
        let input = r#"
        # some comment
        "#;
        AwsConfigFile::parse(input).finish().ok().unwrap();
    }
}
