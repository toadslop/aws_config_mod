//! Traits, functions, etc to help with parsing config files.

use nom::{bytes::complete::tag, error::VerboseError, IResult};
pub type ParserOutput<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

/// Represents any parsable entity in an aws config file
pub(crate) trait Parsable<'a> {
    type Output;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output>;
}

/// Matches a single hash character, '#'
pub(crate) fn hash(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    tag("#")(input)
}

/// Matches a single newline character
pub(crate) fn newline(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    tag("\n")(input)
}

pub(crate) fn to_owned_input(
    error: nom::Err<VerboseError<&str>>,
) -> nom::Err<VerboseError<String>> {
    error.map(|error| VerboseError {
        errors: error
            .errors
            .into_iter()
            .map(|(a, b)| (a.to_string(), b))
            .collect::<Vec<_>>(),
    })
}
