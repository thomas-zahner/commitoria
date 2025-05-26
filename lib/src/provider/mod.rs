use crate::{types::Error, types::Result};
use chrono::NaiveDate;
use serde::Deserialize;
use url::Url;

#[cfg(feature = "git")]
pub mod git;
pub mod gitea;
pub mod github;
pub mod gitlab;

/// Try to parse a `&str` to a `NaiveDate`
fn parse_date(date: &str) -> Result<NaiveDate> {
    const DATE_DESCRIPTION: &'static str = "%Y-%m-%d";

    NaiveDate::parse_from_str(date, DATE_DESCRIPTION)
        .map_err(|e| Error::UnableToParseDate(e.to_string()))
}

/// Information to know how and where to extract data from.
#[derive(Clone, Debug, Deserialize)]
pub struct RepositoryInfo {
    pub url: Url,
    pub user_name: String,
    pub kind: RepositoryKind,
}

#[derive(Clone, Debug, Deserialize)]
pub enum RepositoryKind {
    /// Normal, bare git repository
    BareGitRepository,
    /// Gitea based solutions like Codeberg and Forgejo
    Gitea,
    /// GitLab based solutions (most prominently https://gitlab.com)
    Gitlab,
}
