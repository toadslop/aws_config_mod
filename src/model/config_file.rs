use super::{
    file_content::FileContent, header::Header, Section, SectionName, SectionPath, SectionType,
};
use crate::lexer::Parsable;
use nom::{combinator::eof, error::VerboseError, multi::many_till, IResult};
use std::fmt::Display;

/// Represents a complete aws config file
#[derive(Debug, PartialEq, Eq, Clone, Hash, Default)]
pub struct ConfigFile<'a> {
    /// Represents the content of the file. The content includes the sections of the config
    /// as well as full-line whitespace, which includes comments
    pub(crate) content: Vec<FileContent<'a>>,
    pub(crate) new_content: Vec<FileContent<'a>>,
}

impl<'a> ConfigFile<'a> {
    pub(crate) fn get_section(
        &self,
        section_type: &SectionType,
        section_name: Option<&SectionName>,
    ) -> Option<&Section<'a>> {
        self.content.iter().find_map(|content| match content {
            FileContent::Whitespace(_) => None,
            FileContent::Section(section) => {
                if section.header.section_type == *section_type
                    && section.header.section_name.as_ref() == section_name
                {
                    Some(section)
                } else {
                    None
                }
            }
        })
    }

    pub(crate) fn get_section_mut(
        &mut self,
        section_type: &SectionType,
        section_name: Option<&SectionName>,
    ) -> Option<&mut Section<'a>> {
        self.content.iter_mut().find_map(|content| match content {
            FileContent::Whitespace(_) => None,
            FileContent::Section(section) => {
                if section.header.section_type == *section_type
                    && section.header.section_name.as_ref() == section_name
                {
                    Some(section)
                } else {
                    None
                }
            }
        })
    }

    pub(crate) fn add_section(&mut self, section_path: SectionPath<'a>) -> &'a mut Section {
        let new_section: Section = Section::new(Header::from(section_path.clone()));
        self.new_content.push(FileContent::Section(new_section));

        self.get_section_mut(
            &section_path.section_type,
            section_path.section_name.as_ref(),
        )
        .unwrap()
    }
}

impl<'a> Display for ConfigFile<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.content
                .iter()
                .map(FileContent::to_string)
                .collect::<String>()
        )
    }
}

impl<'a> Parsable<'a> for ConfigFile<'a> {
    type Output = Self;

    fn parse(input: &'a str) -> IResult<&'a str, Self::Output, VerboseError<&'a str>> {
        let (next, (content, _)) = many_till(FileContent::parse, eof)(input)?;
        let config_file = Self {
            content,
            new_content: vec![],
        };

        Ok((next, config_file))
    }
}

#[cfg(test)]
mod test {
    use super::ConfigFile;
    use crate::{lexer::Parsable, model::file_content::FileContent};
    use std::ops::Deref;

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

        let mut sections = config.content.iter();

        let first_newline = sections.next().unwrap();
        match first_newline {
            FileContent::Whitespace(comment) => {
                assert_eq!(comment.deref(), &"\n")
            }
            FileContent::Section(_) => panic!("Should be a comment"),
        }
        let leading_comment = sections.next().unwrap();

        match leading_comment {
            FileContent::Whitespace(comment) => {
                assert_eq!(comment.deref(), &"# I am a leading comment\n")
            }
            FileContent::Section(_) => panic!("Should be a comment"),
        }

        // TODO: finish this test

        let as_string = config.to_string();
        assert_eq!(as_string, SAMPLE_CONFIG_FILE)
    }

    #[test]
    fn parses_sample_config2() {
        let (next, config) = ConfigFile::parse(SAMPLE_CONFIG_FILE_2).expect("Should be valid");
        assert!(next.is_empty());

        let mut sections = config.content.iter();

        let first_newline = sections.next().unwrap();
        match first_newline {
            FileContent::Whitespace(comment) => {
                assert_eq!(comment.deref(), &"\n")
            }
            FileContent::Section(_) => panic!("Should be a comment"),
        }
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
}
