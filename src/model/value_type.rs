use super::{nested_setting::NestedSetting, Value};
use crate::lexer::Parsable;
use nom::{branch::alt, character::complete::line_ending, combinator::map, multi::many0, Parser};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ValueType {
    Single(Value),
    Nested((String, Vec<NestedSetting>)),
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Single(single) => write!(f, "{single}"),
            ValueType::Nested((newline, nested)) => write!(
                f,
                "{newline}{}", // TODO: need to capture the actual newline characters
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
        // let it = map(many0(NestedSetting::parse), Self::Nested)(input)?;
        // let it = map(
        //     line_ending.and(map(many0(NestedSetting::parse), Self::Nested)),
        //     |res| res.1,
        // )(input)?;

        alt((
            map(Value::parse, Self::Single),
            map(
                line_ending.and(many0(NestedSetting::parse)),
                |(newline, nested)| Self::Nested((newline.to_string(), nested)),
            ),
        ))(input)
    }
}
