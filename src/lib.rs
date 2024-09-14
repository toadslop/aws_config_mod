//! # aws_config_mod
//!
//! A missing piece of the Rust AWS SDK. A current missing feature of the SDK is the ability
//! to read and modify AWS config files programatically, such as what we can do with the set command:
//! https://docs.aws.amazon.com/cli/latest/reference/configure/set.html
//!
//! The goal of this library is allowing you to do anything that the `aws configure` command lets
//! you do.
//!
//! The crate is still a work-in-progress, but it can currently parse a config file, modify it,
//! and convert it back to a string. When converting back to a string, will will leave whitespace
//! and comments intact -- the only changes should be the modifications that you added.

mod error;
mod lexer;
mod model;

pub use error::Error;
use lexer::Parsable;
use model::ConfigFile;
pub use model::{Entry, Section, Setting, SubsectionSetting, Value, Whitespace};
use nom::error::VerboseError;
use std::fmt::Display;

/// Represents an AWS config file, as opposed to a credentials file.
///
/// ## Pending Features
///
/// - Load the config file automatically, either from an environment variable or from the default location
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AwsConfigFile<'a>(ConfigFile<'a>);

impl<'a> AwsConfigFile<'a> {
    /// Given a [str], return it parsed as [AwsConfigFile]
    pub fn parse(s: &'a str) -> Result<Self, nom::Err<VerboseError<&'a str>>> {
        let (_, config_file) = ConfigFile::parse(s)?;

        Ok(Self(config_file))
    }

    /// Return the [AwsConfigFile] to its [String] format. This function simply wraps the [Display] implementation.
    pub fn serialize(&self) -> String {
        self.to_string()
    }
}

impl<'a> Display for AwsConfigFile<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
