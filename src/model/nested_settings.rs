//! Contains items related to parsing and stringifying lists of nested settings.

use super::{whitespace::Whitespace, NestedSetting};
use crate::lexer::Parsable;
use nom::{combinator::map, multi::many0, Parser};
use std::{fmt::Display, ops::Deref};

/// Given the configuration file excert below:
///
/// ```
/// [profile test]
/// region = us-west-2
/// s3 =
///    max_concurrent_requests=10
///    max_queue_size=1000
/// ```
///
/// [NestedSettings] would include the content of the last two lines
///
/// This wrapper struct exists to hold whitespace that precedes the list as well as the list itself
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NestedSettings {
    /// Includes any whitespace or comment that occurs in the line preceeding the first nested item.
    pub(crate) leading_whitespace: Whitespace,

    /// The list of [NestedSettings]
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
