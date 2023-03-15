use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    FrankensteinError(#[from] frankenstein::Error),
}
