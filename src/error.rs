//! A custom error type to handle various kinds of parsing errors

use crate::SectionType;
use thiserror::Error;

/// Custom error type. Currently incomplete but will eventually feature better
/// parse-error diagnostics
#[derive(Debug, Error)]
pub enum Error {
    /// Indicates a failure to parse either a configuration file or a path to a setting or section
    #[error("Failed to parse config file:\n\t{0}")]
    ParseError(#[from] nom::Err<nom::error::VerboseError<String>>),

    /// Represents the failure that occurs when trying to create a [crate::SectionPath] for a [crate::SectionType]
    /// which requires a [crate::SectionName]
    #[error("A section name is required for section type '{0}'")]
    SectionNameNeeded(SectionType),
}
