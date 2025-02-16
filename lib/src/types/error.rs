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
    UserNotFound,
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
