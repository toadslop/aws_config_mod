use nom::error::VerboseError;

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
