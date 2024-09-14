use super::file_content::FileContent;
use crate::lexer::Parsable;
use nom::{combinator::eof, error::VerboseError, multi::many_till, IResult};
use std::fmt::Display;

/// Represents a complete aws config file
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConfigFile<'a> {
    /// Represents the content of the file. The content includes the sections of the config
    /// as well as full-line whitespace, which includes comments
    pub content: Vec<FileContent<'a>>,
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
        let config_file = Self { content };

        Ok((next, config_file))
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;

    use super::ConfigFile;
    use crate::{lexer::Parsable, model::file_content::FileContent};

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

    const EMPTY_CONFIG: &str = r#" "#;

    #[test]
    fn parses_sample_config() {
        let (next, config) = ConfigFile::parse(SAMPLE_CONFIG_FILE).expect("Should be valid");
        assert!(next.is_empty());

        let mut sections = config.content.iter();

        let first_newline = sections.next().unwrap();
        match first_newline {
            FileContent::Comment(comment) => {
                assert_eq!(comment.deref(), &"\n")
            }
            FileContent::Section(_) => panic!("Should be a comment"),
        }
        let leading_comment = sections.next().unwrap();

        match leading_comment {
            FileContent::Comment(comment) => {
                assert_eq!(comment.deref(), &"# I am a leading comment\n")
            }
            FileContent::Section(_) => panic!("Should be a comment"),
        }

        // TODO: finish this test

        let as_string = config.to_string();
        assert_eq!(as_string, SAMPLE_CONFIG_FILE)
    }

    #[test]
    fn empty_config_should_pass() {
        let (next, _) = ConfigFile::parse(EMPTY_CONFIG).expect("Should be valid");

        assert!(next.is_empty())
    }
}
