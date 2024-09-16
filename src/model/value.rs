use crate::lexer::{Parsable, ParserOutput};
use nom::{character::complete::none_of, combinator::recognize, multi::many1_count};
use std::{fmt::Display, ops::Deref};

/// Represents the value of a setting. In other words, whatever follows the = sign in a configuration setting.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Hash)]
pub struct Value(String);

impl PartialEq<str> for Value {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<Value> for str {
    fn eq(&self, other: &Value) -> bool {
        self == other.0
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl Deref for Value {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Parsable<'a> for Value {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (input, val) = recognize(many1_count(none_of("#\n\t \r")))(input)?;

        Ok((input, Value(val.to_string())))
    }
}

impl Display for Value {
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
