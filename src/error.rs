//! A custom error type to handle various kinds of parsing errors

use thiserror::Error;

use crate::SectionType;

/// Custom error type. Currently incomplete but will eventually feature better
/// parse-error diagnostics
#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse config file:\n\t{0}")]
    ParseError(#[from] nom::Err<nom::error::VerboseError<String>>),
    #[error("A section name is required for section type '{0}'")]
    SectionNameNeeded(SectionType),
}
