use super::{
    header::Header, whitespace::Whitespace, Section, SectionName, SectionPath, SectionType,
};
use crate::lexer::Parsable;
use nom::{
    combinator::{eof, opt},
    error::VerboseError,
    multi::many0,
    sequence::tuple,
    IResult, Parser,
};
use std::{collections::HashMap, fmt::Display};

/// Represents a complete aws config file
#[derive(Debug, PartialEq, Eq, Clone, Hash, Default)]
pub struct ConfigFile {
    pub(crate) leading_whitespace: Whitespace,
    /// Represents the content of the file. The content includes the sections of the config
    /// as well as full-line whitespace, which includes comments
    pub(crate) sections: Vec<Section>,
    pub(crate) trailing_whitespace: Whitespace,
}

impl ConfigFile {
    pub(crate) fn get_section(
        &self,
        section_type: &SectionType,
        section_name: Option<&SectionName>,
    ) -> Option<&Section> {
        self.sections.iter().find(|section| {
            section.header.section_type == *section_type
                && section.header.section_name.as_ref() == section_name
        })
    }

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

    pub(crate) fn contains_section(&self, section_path: &SectionPath) -> bool {
        self.sections.iter().any(|section| {
            section.get_name() == section_path.section_name.as_ref()
                && *section.get_type() == section_path.section_type
        })
    }

    pub(crate) fn insert_section(&mut self, section_path: &SectionPath) -> &mut Section {
        if !self.contains_section(section_path) {
            let new_section: Section = Section::new(Header::from(section_path.clone()));
            self.sections.push(new_section);
        }

        self.get_section_mut(&section_path.section_type, &section_path.section_name)
            .unwrap()
    }
}

impl Display for ConfigFile {
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

impl<'a> Parsable<'a> for ConfigFile {
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
    use super::ConfigFile;
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
        let (next, config) = ConfigFile::parse(SAMPLE_CONFIG_FILE).expect("Should be valid");
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
        let (next, config) = ConfigFile::parse(SAMPLE_CONFIG_FILE_2).expect("Should be valid");
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
        let (next, _) = ConfigFile::parse(EMPTY_CONFIG).expect("Should be valid");

        assert!(next.is_empty())
    }

    #[test]
    fn empty_string_is_valid_config() {
        ConfigFile::parse("").finish().ok().unwrap();
    }

    #[test]
    fn multiline_whitespace_is_valid() {
        let input = r#"
        # some comment
        "#;
        ConfigFile::parse(input).finish().ok().unwrap();
    }
}
