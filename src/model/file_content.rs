use std::fmt::Display;

use crate::lexer::{Parsable, ParserOutput};

use super::{section::Section, whitespace::Whitespace};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileContent<'a> {
    Comment(Whitespace<'a>),
    Section(Section<'a>),
}

impl<'a> Display for FileContent<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileContent::Comment(comment) => write!(f, "{comment}"),
            FileContent::Section(section) => write!(f, "{section}"),
        }
    }
}

impl<'a> Parsable<'a> for FileContent<'a> {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        if let Ok((next, comment)) = Whitespace::parse(input) {
            return Ok((next, Self::Comment(comment)));
        };

        let (next, section) = Section::parse(input)?;

        Ok((next, Self::Section(section)))
    }
}
