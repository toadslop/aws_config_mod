use crate::lexer::{hash, newline, Parsable, ParserOutput};
use nom::{
    branch::alt,
    character::complete::{crlf, not_line_ending, space0},
    combinator::{eof, opt},
    sequence::pair,
};
use std::{fmt::Display, ops::Deref};

/// Represents meaningless whitespace, including comments. Does not represent meaningful indentation.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default, Hash)]
pub(crate) struct Whitespace(String);

impl Deref for Whitespace {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<str> for Whitespace {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl Display for Whitespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Parsable<'a> for Whitespace {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, leading_spaces) = space0(input)?;
        let (next, maybe_comment) = opt(pair(hash, not_line_ending))(next)?;
        let (hash, rest) = maybe_comment.unwrap_or_default();
        let (next, newline) = alt((newline, crlf, eof))(next)?;
        let whitespace =
            &input[0..(leading_spaces.len() + hash.len() + rest.len() + newline.len())];

        Ok((next, Whitespace(whitespace.to_string())))
    }
}

#[cfg(test)]
mod test {
    use super::Whitespace;
    use crate::lexer::Parsable;

    #[test]
    fn parses_a_comment_with_no_leading_space_and_eof() {
        let comment = "# I am a comment";

        let (rest, com) = Whitespace::parse(comment).expect("Should be ok");

        assert!(rest.is_empty());
        assert_eq!(com, *comment);
        assert_eq!(&com.to_string(), comment)
    }

    #[test]
    fn parses_a_comment_with_no_leading_space_and_newline() {
        let comment = "# I am a comment\n";

        let (rest, com) = Whitespace::parse(comment).expect("Should be ok");

        assert!(rest.is_empty());
        assert_eq!(com, *comment);
        assert_eq!(&com.to_string(), comment)
    }

    #[test]
    fn parses_a_comment_with_no_leading_space_and_newline_carriage_return() {
        let comment = "# I am a comment\r\n";

        let (rest, com) = Whitespace::parse(comment).expect("Should be ok");

        assert!(rest.is_empty());
        assert_eq!(com, *comment);
        assert_eq!(&com.to_string(), comment)
    }

    #[test]
    fn parses_a_comment_with_leading_spaces_and_newline_carriage_return() {
        let comment = "           # I am a comment\r\n";

        let (rest, com) = Whitespace::parse(comment).expect("Should be ok");

        assert!(rest.is_empty());
        assert_eq!(com, *comment);
        assert_eq!(&com.to_string(), comment)
    }

    #[test]
    fn comment_just_spaces_and_newline() {
        let comment = "           \n";

        let (rest, com) = Whitespace::parse(comment).expect("Should be ok");

        assert!(rest.is_empty());
        assert_eq!(com, *comment);
        assert_eq!(&com.to_string(), comment)
    }
}
