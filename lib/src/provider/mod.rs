use crate::{source::DataSource, types::ContributionActivity, types::Error, types::Result};
use chrono::NaiveDate;
use std::future::Future;

#[cfg(feature = "git")]
pub mod git;
pub mod gitea;
pub mod github;
pub mod gitlab;

pub trait GitProvider {
    fn fetch<S: DataSource>(
        data_source: S,
        user_name: String,
    ) -> impl Future<Output = Result<ContributionActivity>>;
}

fn parse_date(date: &str) -> Result<NaiveDate> {
    const DATE_DESCRIPTION: &'static str = "%Y-%m-%d";

    NaiveDate::parse_from_str(date, DATE_DESCRIPTION)
        .map_err(|e| Error::UnableToParseDate(e.to_string()))
}
