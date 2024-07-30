use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("Failed to parse year: {0}")]
    YearParseError(#[from] std::num::ParseIntError),

    #[error("Failed to create date")]
    DateCreationError,

    #[error("GitHub API error: {0}")]
    GitHubApiError(#[from] octocrab::Error),

    #[error("Unexpected response format")]
    UnexpectedResponseFormat,
}
