use crate::{source::DataSource, ContributionActivity};
use scraper::error::SelectorErrorKind;
use time::{format_description::BorrowedFormatItem, macros::format_description, Date};

pub mod github;
pub mod gitlab;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    SelectorError(String),
    AttributeMissing,
    TooltipMissing,
    UnexpectedTooltipMessage(String),
    UnableToParseDate(String),
    UnableToParseJson(String),
}

impl From<SelectorErrorKind<'_>> for Error {
    fn from(value: SelectorErrorKind<'_>) -> Self {
        Self::SelectorError(value.to_string())
    }
}

pub trait GitProvider {
    fn fetch<S: DataSource>(data_source: S, user_name: String) -> Result<ContributionActivity>;
}

fn parse_date(date: &str) -> Result<Date> {
    const DATE_DESCRIPTION: &'static [BorrowedFormatItem<'static>] =
        format_description!("[year]-[month]-[day]");

    Date::parse(date, DATE_DESCRIPTION).map_err(|e| Error::UnableToParseDate(e.to_string()))
}
