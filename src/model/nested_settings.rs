use super::{whitespace::Whitespace, NestedSetting};
use crate::lexer::Parsable;
use nom::{combinator::map, multi::many0, Parser};
use std::{fmt::Display, ops::Deref};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NestedSettings {
    pub(crate) leading_whitespace: Whitespace,
    pub(crate) nested_settings: Vec<NestedSetting>,
}

impl Deref for NestedSettings {
    type Target = Vec<NestedSetting>;

    fn deref(&self) -> &Self::Target {
        &self.nested_settings
    }
}

impl Display for NestedSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.leading_whitespace,
            self.nested_settings
                .iter()
                .map(NestedSetting::to_string)
                .collect::<String>()
        )
    }
}

impl<'a> Parsable<'a> for NestedSettings {
    type Output = Self;

    fn parse(input: &'a str) -> crate::lexer::ParserOutput<'a, Self::Output> {
        map(
            Whitespace::parse.and(many0(NestedSetting::parse)),
            Self::from,
        )(input)
    }
}

impl From<(Whitespace, Vec<NestedSetting>)> for NestedSettings {
    fn from((leading_whitespace, nested_settings): (Whitespace, Vec<NestedSetting>)) -> Self {
        Self {
            leading_whitespace,
            nested_settings,
        }
    }
}
