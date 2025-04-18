use super::Result;
use crate::types::{ContributionActivity, Error, YEAR};
use chrono::{DateTime, NaiveDate};
use git2::{build::RepoBuilder, FetchOptions, Sort};
use serde::Deserialize;
use std::{collections::BTreeMap, path::PathBuf};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
pub struct RepositoryInfo {
    url: String,
    user_name: String,
}

/// Represents a Git repository to be cloned and analysed
pub struct Repository(PathBuf);

impl Drop for Repository {
    fn drop(&mut self) {
        if let Err(e) = std::fs::remove_dir_all(&self.0) {
            eprintln!("Failed to remove directory when repository was dropped: {e}");
        }
    }
}

impl Repository {
    pub fn new() -> Self {
        let path_string = format!("/tmp/{}", Uuid::new_v4());
        Self(PathBuf::from(path_string))
    }

    /// Clone repository and extract `ContributionActivity`.
    /// This function takes ownership of `self` so that the created directory is cleand up afterwards.
    pub async fn fetch(self, info: RepositoryInfo) -> Result<ContributionActivity> {
        let one_year_ago = chrono::Local::now().date_naive() - YEAR;
        self.fetch_since(info, one_year_ago).await
    }

    async fn fetch_since(
        self,
        info: RepositoryInfo,
        since: NaiveDate,
    ) -> Result<ContributionActivity> {
        let options = FetchOptions::new();
        let mut builder = RepoBuilder::new();
        builder.fetch_options(options).bare(true);
        // TODO: ideally we want to use the option `--shallow-since "1 year"`
        // But not yet supported: https://github.com/libgit2/libgit2/issues/6611

        let repo = builder.clone(&info.url, &self.0)?;
        let mut revwalk = repo.revwalk()?;

        revwalk.set_sorting(Sort::TIME)?;
        revwalk.push_head()?;

        let mut result = BTreeMap::new();

        for rev in revwalk.into_iter() {
            let rev = *rev.as_ref()?;
            let commit = repo.find_commit(rev)?;
            let commit_time = DateTime::from_timestamp(commit.time().seconds(), 0)
                .ok_or(Error::UnableToParseDate(
                    "Invalid timestamp encountered".into(),
                ))?
                .date_naive();

            if commit_time >= since
                && (commit.author().name() == Some(&info.user_name)
                    || commit.author().email() == Some(&info.user_name))
            {
                *result.entry(commit_time).or_insert(0) += 1;
            }
        }

        Ok(result.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::provider::git::{Repository, RepositoryInfo};
    use chrono::NaiveDate;

    #[tokio::test]
    async fn git_repository() {
        let repository = Repository::new();
        let since = NaiveDate::from_ymd_opt(2024, 01, 01).unwrap();

        let result = repository
            .fetch_since(
                RepositoryInfo {
                    url: "https://github.com/thomas-zahner/commitoria".into(),
                    user_name: "Thomas Zahner".into(),
                },
                since,
            )
            .await
            .unwrap();

        assert!(result.contribution_count() > 100);
        assert_eq!(
            result.get(&NaiveDate::from_ymd_opt(2024, 12, 12).unwrap()),
            None
        );
        assert_eq!(
            result.get(&NaiveDate::from_ymd_opt(2024, 12, 13).unwrap()),
            Some(3)
        );
    }
}
