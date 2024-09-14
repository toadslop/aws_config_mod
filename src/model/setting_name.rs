use crate::lexer::{Parsable, ParserOutput};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, combinator::recognize,
    multi::many0_count,
};
use std::fmt::Display;

/// Represents the name of a setting; in other words, the part that comes before the '=' sign.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct SettingName<'a>(&'a str);

impl<'a> Display for SettingName<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Parsable<'a> for SettingName<'a> {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (input, setting_name) = recognize(many0_count(alt((alpha1, tag("_")))))(input)?;
        Ok((input, Self(setting_name)))
    }
}

#[cfg(test)]
mod test {
    use crate::{lexer::Parsable, model::setting_name::SettingName};

    #[test]
    fn parses_valid_setting_names() {
        let names = [
            "aws_access_key_id",
            "aws_secret_access_key",
            "region",
            "endpoint_url",
        ];

        for name in names {
            let (input, value) = SettingName::parse(name).expect("Should be valid");
            assert!(input.is_empty());
            assert_eq!(value.0, name);
            assert_eq!(&value.to_string(), name)
        }
    }
}
