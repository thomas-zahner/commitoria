use super::Result;
use crate::types::{ContributionActivity, Error, YEAR};
use chrono::{DateTime, NaiveDate};
use git2::{build::RepoBuilder, FetchOptions, RemoteCallbacks, Sort};
use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::Mutex,
    time::{Duration, Instant},
};
use tokio::task;
use url::Url;
use uuid::Uuid;

const CLONE_TIMEOUT: Duration = Duration::from_millis(5_000);

/// Represents a Git repository to be cloned and analysed
pub struct Repository(Mutex<git2::Repository>);

impl Drop for Repository {
    fn drop(&mut self) {
        match self.0.lock() {
            Err(e) => eprintln!("{e}"),
            Ok(repository) => {
                let path = repository.path();
                if let Err(e) = std::fs::remove_dir_all(path) {
                    eprintln!("Failed to remove directory when repository was dropped: {e}");
                }
            }
        }
    }
}

fn is_timed_out(begin: Instant) -> bool {
    Instant::now() - begin > CLONE_TIMEOUT
}

fn register_timeout_callback(options: &mut FetchOptions<'_>, begin: Instant) {
    let mut callbacks = RemoteCallbacks::default();
    callbacks.transfer_progress(move |_| !is_timed_out(begin));
    options.remote_callbacks(callbacks);
}

impl Repository {
    /// Clones the specified Git repository by URL
    pub async fn new(url: Url) -> Result<Self> {
        let path = PathBuf::from(format!("/tmp/{}", Uuid::new_v4()));
        let mut builder = RepoBuilder::new();
        let mut options = FetchOptions::new();

        let begin = Instant::now();
        register_timeout_callback(&mut options, begin);
        builder.fetch_options(options).bare(true);

        // TODO: ideally we want to use the option `--shallow-since "1 year"`
        // But not yet supported: https://github.com/libgit2/libgit2/issues/6611

        let result = task::block_in_place(move || builder.clone(url.as_str(), &path.clone()));
        let result = result.map_err(|e| match e {
            git2::Error { .. } if is_timed_out(begin) => Error::RepositoryCloningTimedOut,
            e => e.into(),
        });

        Ok(Self(Mutex::new(result?)))
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

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn git_repository() {
        let repository = Repository::new(
            "https://github.com/thomas-zahner/commitoria"
                .try_into()
                .unwrap(),
        )
        .await;
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
