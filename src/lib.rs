//! # aws_config_mod
//!
//! A missing piece of the Rust AWS SDK. A current missing feature of the SDK is the ability
//! to read and modify AWS config files programatically, such as what we can do with the set command:
//! <https://docs.aws.amazon.com/cli/latest/reference/configure/set.html>
//!
//! The goal of this library is allowing you to add, modify, and remove AWS CLI configuration settings for
//! existing configuration files to to generate new ones while any existing whitespace or comments
//! in the original file.
//!
//! This crate is still a work-in-progress, so if there are any features that you need right away, please
//! open an issue and they will be prioritized.
//!
//! ## Usage
//!
//! ```
//! // Assuming you already read the configuration file to a string
//! let config_content = r#"
//! [profile A]
//! credential_source = Ec2InstanceMetadata
//! endpoint_url = https://profile-a-endpoint.aws/
//!
//! [profile B]
//! source_profile = A
//! role_arn = arn:aws:iam::123456789012:role/roleB
//! services = profileB
//!
//! [services profileB]
//! ec2 =
//!   endpoint_url = https://profile-b-ec2-endpoint.aws"#;
//!
//! let mut config = AwsConfigFile::parse(SAMPLE_FILE).expect("Sample file should be valid");
//!
//! let setting_path = SettingPath::try_from("profile.A.credential_source").expect("Should parse");
//! config.set(setting_path, Value::from("my-new-credential-source"));
//! let stringified = config.to_string();
//! // Write the content back to your file
//! ```

mod error;
mod lexer;
mod model;

pub use error::Error;
use lexer::Parsable;
use model::ConfigFile;
pub use model::{
    NestedSetting, NestedSettingPath, Section, SectionName, SectionPath, SectionType, Setting,
    SettingName, SettingPath, Value, ValueType,
};
use nom::error::VerboseError;
use std::fmt::Display;

/// Represents an AWS config file -- it does not include the content of a credentials file.
/// Internally it tracks the whitespace so calling [AwsConfigFile::to_string] will reproduce
/// the original whitespace and comments of the file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AwsConfigFile(ConfigFile);

impl AwsConfigFile {
    /// Given a [str], return it parsed as [AwsConfigFile]
    pub fn parse(s: &str) -> Result<Self, nom::Err<VerboseError<&str>>> {
        let (_, config_file) = ConfigFile::parse(s)?;

        Ok(Self(config_file))
    }

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

        self.0.get_section(section_type, section_name.as_ref())
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
        let section = match self.0.get_section_mut(
            &setting_path.section_path.section_type,
            &setting_path.section_path.section_name,
        ) {
            Some(section) => section,
            None => self.0.insert_section(&setting_path.section_path),
        };

        section.set(setting_path.setting_name, value);
    }
}

impl Display for AwsConfigFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
