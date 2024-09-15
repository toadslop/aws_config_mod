use crate::lexer::{Parsable, ParserOutput};
use nom::{character::complete::none_of, combinator::recognize, multi::many1_count};
use std::{fmt::Display, ops::Deref};

/// Represents the value of a setting. In other words, whatever follows the = sign in a configuration setting.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Value<'a>(&'a str);

impl<'a> Deref for Value<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Parsable<'a> for Value<'a> {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (input, val) = recognize(many1_count(none_of("#\n\t \r")))(input)?;

        Ok((input, Value(val)))
    }
}

impl<'a> Display for Value<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Parsable;

    use super::Value;

    #[test]
    fn parses_access_key_as_value() {
        let value = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";

        let (input, val) = Value::parse(value).expect("Should be valid");

        assert!(input.is_empty());
        assert_eq!(val.0, value);
        assert_eq!(&val.to_string(), value)
    }

    #[test]
    fn parses_url_as_value() {
        let value = "https://profile-b-ec2-endpoint.aws";

        let (input, val) = Value::parse(value).expect("Should be valid");

        assert!(input.is_empty());
        assert_eq!(val.0, value);
        assert_eq!(&val.to_string(), value)
    }

    #[test]
    fn parses_arn_as_value() {
        let value = "arn:aws:iam::123456789012:role/roleB";

        let (input, val) = Value::parse(value).expect("Should be valid");

        assert!(input.is_empty());
        assert_eq!(val.0, value);
        assert_eq!(&val.to_string(), value)
    }
}
