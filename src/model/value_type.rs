use super::{
    nested_setting::NestedSetting, nested_settings::NestedSettings, whitespace::Whitespace, Value,
};
use crate::lexer::Parsable;
use nom::{combinator::opt, multi::many1};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ValueType<'a> {
    Single(Value<'a>),
    Nested(NestedSettings<'a>),
}

impl<'a> Display for ValueType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Single(single) => write!(f, "{single}"),
            ValueType::Nested(nested) => write!(f, "{}", nested),
        }
    }
}

impl<'a> Parsable<'a> for ValueType<'a> {
    type Output = Self;

    fn parse(input: &'a str) -> crate::lexer::ParserOutput<'a, Self::Output> {
        let (next, whitespace) = opt(Whitespace::parse)(input)?;

        if let Some(whitespace) = whitespace {
            let (next, settings) = many1(NestedSetting::parse)(next)?;
            let settings = NestedSettings {
                prev_line_whitespace: whitespace,
                settings,
            };
            Ok((next, ValueType::Nested(settings)))
        } else {
            let (next, value) = Value::parse(next)?;
            Ok((next, ValueType::Single(value)))
        }
    }
}
