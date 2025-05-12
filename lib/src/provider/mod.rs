use crate::{types::Error, types::Result};
use chrono::NaiveDate;
use serde::Deserialize;
use url::Url;

#[cfg(feature = "git")]
pub mod git;
pub mod gitea;
pub mod github;
pub mod gitlab;

fn parse_date(date: &str) -> Result<NaiveDate> {
    const DATE_DESCRIPTION: &'static str = "%Y-%m-%d";

    NaiveDate::parse_from_str(date, DATE_DESCRIPTION)
        .map_err(|e| Error::UnableToParseDate(e.to_string()))
}

#[derive(Clone, Debug, Deserialize)]
pub struct RepositoryInfo {
    pub url: Url,
    pub user_name: String,
}
