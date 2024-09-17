use super::{nested_setting::NestedSetting, Value};
use crate::lexer::Parsable;
use nom::{branch::alt, combinator::map, multi::many0};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ValueType {
    Single(Value),
    Nested(Vec<NestedSetting>),
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Single(single) => write!(f, "{single}"),
            ValueType::Nested(nested) => write!(
                f,
                "{}",
                nested
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
            map(many0(NestedSetting::parse), Self::Nested),
        ))(input)
    }
}
