use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error<'a> {
    #[error("")]
    UnknownError(nom::Err<nom::error::Error<&'a str>>),
}
