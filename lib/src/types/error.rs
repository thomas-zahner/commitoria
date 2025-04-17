use crate::svg::svg_renderer::BuilderError;
use reqwest::StatusCode;
use scraper::error::SelectorErrorKind;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SelectorError(String),
    AttributeMissing,
    TooltipMissing,
    UnexpectedTooltipMessage(String),
    UnableToParseDate(String),
    UnableToParseJson(String),
    ReqwestError(String),
    GitError(String),
    UserNotFound,
    BuilderError(BuilderError),
}

impl From<BuilderError> for Error {
    fn from(error: BuilderError) -> Self {
        Self::BuilderError(error)
    }
}

impl From<Error> for (StatusCode, String) {
    fn from(error: Error) -> Self {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error))
    }
}

impl From<SelectorErrorKind<'_>> for Error {
    fn from(value: SelectorErrorKind<'_>) -> Self {
        Self::SelectorError(value.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::UnableToParseJson(value.to_string())
    }
}

#[cfg(feature = "git")]
impl From<git2::Error> for Error {
    fn from(value: git2::Error) -> Self {
        Self::GitError(value.to_string())
    }
}

#[cfg(feature = "git")]
impl From<&git2::Error> for Error {
    fn from(value: &git2::Error) -> Self {
        Self::GitError(value.to_string())
    }
}
