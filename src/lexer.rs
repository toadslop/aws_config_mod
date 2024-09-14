use nom::{bytes::complete::tag, error::VerboseError, IResult};

pub type ParserOutput<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

/// Represents any parsable entity in an aws config file
pub trait Parsable<'a> {
    type Output;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output>;
}

/// Matches a single hash character, '#'
pub fn hash(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    tag("#")(input)
}

/// Matches a single newline character
pub fn newline(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    tag("\n")(input)
}
