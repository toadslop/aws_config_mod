use crate::lexer::{hash, newline, Parsable, ParserOutput};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, crlf, not_line_ending, space0},
    combinator::{eof, not, opt, recognize},
    error::VerboseError,
    multi::separated_list0,
    sequence::pair,
    IResult, Parser,
};
use std::{fmt::Display, ops::Deref};

/// Represents meaningless whitespace, including comments. Does not represent meaningful indentation.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default, Hash)]
pub(crate) struct Whitespace(pub(crate) String);

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
        let (next, whitespace) =
            recognize(separated_list0(line_end, space0.and(opt(comment)))).parse(input)?;

        Ok((next, Whitespace(whitespace.to_string())))
    }
}

fn comment(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    recognize(hash.and(not_line_ending)).parse(input)
}

fn line_end(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    alt((newline, crlf))(input)
}

#[cfg(test)]
mod test {
    use super::Whitespace;
    use crate::lexer::Parsable;

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

    #[test]
    fn multilines_of_whitespace() {
        let comment = r#"
    # hello comment
               # more comment stuff
            "#;

        let (rest, com) = Whitespace::parse(comment).expect("Should be ok");

        assert!(rest.is_empty());
        assert_eq!(com, *comment);
        assert_eq!(&com.to_string(), comment)
    }
}
