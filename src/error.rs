use failure::Error;

#[derive(Debug, Fail)]
pub enum HolochainError {
    #[fail(display = "CLI error: {}", _0)]
    Cli(CliError),
    #[fail(display = "Error: {}", _0)]
    Default(Error),
}

#[derive(Debug, Fail)]
pub enum CliError {
    #[fail(display = "unknown language")]
    UnknownLanguage,
}

pub type DefaultResult<T> = Result<T, Error>;
