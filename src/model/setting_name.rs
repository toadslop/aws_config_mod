use crate::lexer::{Parsable, ParserOutput};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::alphanumeric1, combinator::recognize,
    multi::many0_count,
};
use std::{borrow::Cow, fmt::Display, ops::Deref};

/// Represents the name of a setting; in other words, the part that comes before the '=' sign.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct SettingName<T>(pub(crate) T)
where
    T: Display;

impl PartialEq<str> for SettingName<Cow<'_, str>> {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<SettingName<Cow<'_, str>>> for str {
    fn eq(&self, other: &SettingName<Cow<'_, str>>) -> bool {
        self == other.0
    }
}

impl<T> Display for SettingName<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> Deref for SettingName<T>
where
    T: Display,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Parsable<'a> for SettingName<Cow<'a, str>> {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (input, setting_name) = recognize(many0_count(alt((alphanumeric1, tag("_")))))(input)?;
        Ok((input, Self(Cow::Borrowed(setting_name))))
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
