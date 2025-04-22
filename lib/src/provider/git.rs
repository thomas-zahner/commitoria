use super::Result;
use crate::types::{ContributionActivity, Error, YEAR};
use chrono::{DateTime, NaiveDate};
use git2::{build::RepoBuilder, FetchOptions, Sort};
use serde::Deserialize;
use std::{collections::BTreeMap, path::PathBuf, sync::Mutex};
use tokio::task;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
pub struct RepositoryInfo {
    pub url: String,
    pub user_name: String,
}

/// Represents a Git repository to be cloned and analysed
pub struct Repository(Mutex<git2::Repository>);

impl Drop for Repository {
    fn drop(&mut self) {
        if let Err(e) = std::fs::remove_dir_all(self.0.lock().unwrap().path()) {
            eprintln!("Failed to remove directory when repository was dropped: {e}");
        }
    }
}

impl Repository {
    /// Clones the specified Git repository by URL
    pub async fn new(url: String) -> Result<Self> {
        let path = PathBuf::from(format!("/tmp/{}", Uuid::new_v4()));
        let result = task::spawn_blocking(move || {
            let mut builder = RepoBuilder::new();
            let options = FetchOptions::new();
            builder.fetch_options(options).bare(true);

            // TODO: ideally we want to use the option `--shallow-since "1 year"`
            // But not yet supported: https://github.com/libgit2/libgit2/issues/6611

            builder.clone(&url, &path)
        });
        Ok(Self(Mutex::new(result.await.unwrap()?)))
    }

    /// Get activity of the specified `user` in the last year.
    /// `user` matches both an author's name or email.
    pub async fn get_activity(&self, user: String) -> Result<ContributionActivity> {
        let one_year_ago = chrono::Local::now().date_naive() - YEAR;
        self.get_activity_since(user, one_year_ago).await
    }

    async fn get_activity_since(
        &self,
        user: String,
        since: NaiveDate,
    ) -> Result<ContributionActivity> {
        let repository = self.0.lock().unwrap();
        let mut revwalk = repository.revwalk()?;

        revwalk.set_sorting(Sort::TIME)?;
        revwalk.push_head()?;

        let mut result = BTreeMap::new();

        for rev in revwalk.into_iter() {
            let rev = *rev.as_ref()?;
            let commit = repository.find_commit(rev)?;
            let commit_time = DateTime::from_timestamp(commit.time().seconds(), 0)
                .ok_or(Error::UnableToParseDate(
                    "Invalid timestamp encountered".into(),
                ))?
                .date_naive();

            if commit_time >= since
                && (commit.author().name() == Some(&user) || commit.author().email() == Some(&user))
            {
                *result.entry(commit_time).or_insert(0) += 1;
            }
        }

        Ok(result.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::provider::git::Repository;
    use chrono::NaiveDate;

    #[tokio::test]
    async fn git_repository() {
        let repository =
            Repository::new("https://github.com/thomas-zahner/commitoria".into()).await;
        let since = NaiveDate::from_ymd_opt(2024, 01, 01).unwrap();

        let result = repository
            .unwrap()
            .get_activity_since("Thomas Zahner".into(), since)
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
