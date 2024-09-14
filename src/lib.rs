mod error;
mod lexer;
mod model;

pub use error::Error;
use lexer::Parsable;
use model::ConfigFile;
pub use model::{Entry, Whitespace};
use nom::error::VerboseError;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AwsConfigFile<'a>(ConfigFile<'a>);

impl<'a> AwsConfigFile<'a> {
    pub fn parse(s: &'a str) -> Result<Self, nom::Err<VerboseError<&'a str>>> {
        let (_, config_file) = ConfigFile::parse(s)?;

        Ok(Self(config_file))
    }
}

impl<'a> Display for AwsConfigFile<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
