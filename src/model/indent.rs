use crate::lexer::{Parsable, ParserOutput};
use nom::{
    character::complete::{alphanumeric1, space1},
    combinator::{peek, recognize},
    multi::many0_count,
    sequence::tuple,
};
use std::fmt::Display;

/// Represents non-comment whitespace at the head of start of a [crate::Setting]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
pub(crate) struct Indent(String);

impl PartialEq<&str> for Indent {
    fn eq(&self, other: &&str) -> bool {
        self.0.as_str() == *other
    }
}

impl PartialEq<Indent> for &str {
    fn eq(&self, other: &Indent) -> bool {
        *self == other.0.as_str()
    }
}

impl Display for Indent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Parsable<'a> for Indent {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, ws) = recognize(tuple((many0_count(space1), peek(alphanumeric1))))(input)?;

        Ok((next, Self(ws.to_string())))
    }
}

#[cfg(test)]
mod test {
    use super::Indent;
    use crate::lexer::Parsable;

    #[test]
    fn empty_string_is_not_indent() {
        let input = "";

        Indent::parse(input).expect_err("Should not parse an empty string into an indent");
    }

    #[test]
    fn spaces_with_no_content_is_not_indent() {
        let input = "   ";

        Indent::parse(input).expect_err("Should not parse only spaces as an indent");
    }

    #[test]
    fn beginning_of_a_comment_is_not_indent() {
        let input = "   #";

        Indent::parse(input).expect_err("Should not parse spaces followed by # as an indent");
    }

    #[test]
    fn indent_followed_by_a_setting_is_valid() {
        let input = "  setting_name = setting_value";

        let (next, indent) = Indent::parse(input).expect("Should be valid");
        assert_eq!(next, "setting_name = setting_value");
        assert_eq!(indent, "  ")
    }
}
