//! Contains items related to parsing and stringifying setting names. Setting names are the
//! identifiers for the setting values in a configuration file.

use crate::lexer::{to_owned_input, Parsable, ParserOutput};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::alphanumeric1, combinator::recognize,
    multi::many0_count,
};
use std::{fmt::Display, ops::Deref, str::FromStr};

/// Represents the name of a setting; in other words, the part that comes before the '=' sign.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct SettingName(String);

impl PartialEq<str> for SettingName {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<SettingName> for str {
    fn eq(&self, other: &SettingName) -> bool {
        self == other.0
    }
}

impl Display for SettingName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for SettingName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for SettingName {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
            .map(|a| a.1)
            .map_err(to_owned_input)
            .map_err(crate::Error::from)
    }
}

impl<'a> Parsable<'a> for SettingName {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (input, setting_name) = recognize(many0_count(alt((alphanumeric1, tag("_")))))(input)?;
        Ok((input, Self(setting_name.to_string())))
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
