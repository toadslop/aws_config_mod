use super::{section::Section, whitespace::Whitespace};
use crate::lexer::{Parsable, ParserOutput};
use std::fmt::Display;

/// Represents either a line containing only whitespace (or a comment), or a complete section.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FileContent {
    /// A line of whitespace, potentially including a comment
    Whitespace(Whitespace),

    /// A complete section of a config file
    Section(Section),
}

impl Display for FileContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileContent::Whitespace(comment) => write!(f, "{comment}"),
            FileContent::Section(section) => write!(f, "{section}"),
        }
    }
}

impl<'a> Parsable<'a> for FileContent {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        if let Ok((next, comment)) = Whitespace::parse(input) {
            return Ok((next, Self::Whitespace(comment)));
        };

        let (next, section) = Section::parse(input)?;

        Ok((next, Self::Section(section)))
    }
}
