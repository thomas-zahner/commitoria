use super::{GitProvider, Result};
use crate::types::ContributionActivity;
use git2::{build::RepoBuilder, FetchOptions, Repository, Sort};
use std::{
    path::Path,
    time::{Instant, SystemTime},
};

pub struct Git {}

impl GitProvider for Git {
    async fn fetch<S: crate::source::DataSource>(
        data_source: S,
        user_name: String,
    ) -> Result<ContributionActivity> {
        let url = "https://github.com/alexcrichton/git2-rs";

        // TODO: is it possible to use the following?
        // --shallow-since "1 year"

        let options = FetchOptions::new();
        let mut builder = RepoBuilder::new();
        builder.fetch_options(options).bare(true);

        let repo = builder.clone(url, Path::new("/tmp/git2-rs")).unwrap();
        let mut revwalk = repo.revwalk().unwrap();

        revwalk.set_sorting(Sort::TIME).unwrap();
        revwalk.push_head().unwrap();

        for rev in revwalk {
            let commit = repo.find_commit(rev.unwrap()).unwrap();
            dbg!(commit.time().seconds());
            dbg!(commit.message());

            // todo: use chrono or time crate for subtracting one year
            const ONE_YEAR: usize = 60 * 60 * 24 * 365;
            let x = SystemTime::from(ONE_YEAR);
            let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) - ONE_YEAR;
            dbg!(time);
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::FixtureDataSource;

    #[tokio::test]
    async fn git_repository() {
        let result = Git::fetch(FixtureDataSource {}, "".into()).await.unwrap();
    }
}
