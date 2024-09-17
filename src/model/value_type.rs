use super::{nested_setting::NestedSetting, nested_settings::NestedSettings, Value};
use crate::lexer::Parsable;
use nom::{branch::alt, combinator::map};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ValueType {
    Single(Value),
    Nested(NestedSettings),
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Single(single) => write!(f, "{single}"),
            ValueType::Nested(NestedSettings {
                leading_whitespace,
                nested_settings,
            }) => write!(
                f,
                "{leading_whitespace}{}", // TODO: need to capture the actual newline characters
                nested_settings
                    .iter()
                    .map(NestedSetting::to_string)
                    .collect::<String>()
            ),
        }
    }
}

impl<'a> Parsable<'a> for ValueType {
    type Output = Self;

    fn parse(input: &'a str) -> crate::lexer::ParserOutput<'a, Self::Output> {
        alt((
            map(Value::parse, Self::Single),
            map(NestedSettings::parse, Self::Nested),
        ))(input)
    }
}
