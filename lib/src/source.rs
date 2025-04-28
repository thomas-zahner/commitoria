use std::future::Future;

use crate::types::{Error, Result};

pub enum Source {
    GithubUser(String),
    GitlabUser(String),
    GiteaUser { user: String, hostname: String },
}

pub trait DataSource {
    fn fetch(&self, source: Source) -> impl Future<Output = Result<String>>;
}

pub struct ReqwestDataSource {}

impl DataSource for ReqwestDataSource {
    async fn fetch(&self, source: Source) -> Result<String> {
        let url = match source {
            Source::GithubUser(user) => format!("https://github.com/users/{user}/contributions"),
            Source::GitlabUser(user) => format!("https://gitlab.com/users/{user}/calendar.json"),
            Source::GiteaUser { user, hostname } => {
                format!("https://{}/{}?tab=activity", hostname, user)
            }
        };

        Ok(reqwest::get(url)
            .await?
            .error_for_status()
            .map_err(|e| match e {
                e if e.is_status() => Error::UserNotFound,
                e => e.into(),
            })?
            .text()
            .await?)
    }
}

#[cfg(test)]
pub struct FixtureDataSource {}

#[cfg(test)]
impl DataSource for FixtureDataSource {
    async fn fetch(&self, source: Source) -> Result<String> {
        let fixture_path = match source {
            Source::GithubUser(_) => "fixtures/github.html",
            Source::GitlabUser(_) => "fixtures/gitlab.json",
            Source::GiteaUser {
                user: _,
                hostname: _,
            } => "fixtures/gitea.html",
        };

        Ok(std::fs::read_to_string(fixture_path).expect("Unable to read file"))
    }
}
