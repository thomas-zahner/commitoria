use super::{GitProvider, Result};
use crate::types::ContributionActivity;
use git2::{build::RepoBuilder, FetchOptions, Repository, Sort};
use std::path::Path;

pub struct Git {}

impl GitProvider for Git {
    async fn fetch<S: crate::source::DataSource>(
        data_source: S,
        user_name: String,
    ) -> Result<ContributionActivity> {
        let url = "https://github.com/alexcrichton/git2-rs";

        // TODO: is it possible to use the following?
        // --bare
        // --shallow-since "1 year"

        let options = FetchOptions::new();
        let mut builder = RepoBuilder::new();
        builder.fetch_options(options).bare(true);

        let repo = builder.clone(url, Path::new("/tmp/git2-rs")).unwrap();
        let mut revwalk = repo.revwalk().unwrap();

        revwalk.set_sorting(Sort::TIME).unwrap();
        revwalk.push_head().unwrap();

        for commit in revwalk {
            dbg!(commit.unwrap());
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
