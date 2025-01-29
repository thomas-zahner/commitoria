use crate::{source::DataSource, types::ContributionActivity, types::Error, types::Result};
use std::future::Future;
use time::{format_description::BorrowedFormatItem, macros::format_description, Date};

pub mod github;
pub mod gitlab;

pub trait GitProvider {
    fn fetch<S: DataSource>(
        data_source: S,
        user_name: String,
    ) -> impl Future<Output = Result<ContributionActivity>>;
}

fn parse_date(date: &str) -> Result<Date> {
    const DATE_DESCRIPTION: &'static [BorrowedFormatItem<'static>] =
        format_description!("[year]-[month]-[day]");

    Date::parse(date, DATE_DESCRIPTION).map_err(|e| Error::UnableToParseDate(e.to_string()))
}
