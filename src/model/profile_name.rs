use std::{fmt::Display, ops::Deref};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::alphanumeric1, combinator::recognize,
    multi::many1_count,
};

use crate::lexer::{Parsable, ParserOutput};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct ProfileName<'a>(&'a str);

impl<'a> Display for ProfileName<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Deref for ProfileName<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Parsable<'a> for ProfileName<'a> {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, profile_name) =
            recognize(many1_count(alt((alphanumeric1, tag("_"), tag("-")))))(input)?;

        Ok((next, Self(profile_name)))
    }
}
