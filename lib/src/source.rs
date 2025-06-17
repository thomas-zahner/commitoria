use std::future::Future;

use reqwest::{IntoUrl, StatusCode};

use crate::types::{Error, Result};

pub trait DataSource {
    fn fetch<T: IntoUrl>(&self, source: T) -> impl Future<Output = Result<String>>;
}

pub struct ReqwestDataSource {}

impl DataSource for ReqwestDataSource {
    async fn fetch<T: IntoUrl>(&self, source: T) -> Result<String> {
        Ok(reqwest::get(source)
            .await?
            .error_for_status()
            .map_err(|e| match e.status() {
                Some(StatusCode::NOT_FOUND) => Error::UserNotFound,
                _ => e.into(),
            })?
            .text()
            .await?)
    }
}

#[cfg(test)]
pub enum FixtureDataSource {
    GithubUser,
    GitlabUser,
    GiteaUser,
}

#[cfg(test)]
impl DataSource for FixtureDataSource {
    async fn fetch<T: IntoUrl>(&self, _: T) -> Result<String> {
        let fixture_path = match self {
            Self::GithubUser => "fixtures/github.html",
            Self::GitlabUser => "fixtures/gitlab.json",
            Self::GiteaUser => "fixtures/gitea.html",
        };

        Ok(std::fs::read_to_string(fixture_path).expect("Unable to read file"))
    }
}
