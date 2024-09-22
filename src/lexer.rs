//! Traits, functions, etc to help with parsing config files.

use nom::{bytes::complete::tag, error::VerboseError, IResult};

/// The return type from a parser
pub type ParserOutput<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

/// Represents any parsable entity in an aws config file
pub(crate) trait Parsable<'a> {
    /// The output of the parser
    type Output;

    /// Defines how to parse the output
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

/// Matches a single newline character
pub(crate) fn equal(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    tag("=")(input)
}

/// A helper to convert [nom::Err<VerboseError<&str>>] to an owned type
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
