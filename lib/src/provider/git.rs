use super::{GitProvider, Result};
use crate::types::ContributionActivity;
use chrono::{DateTime, Months};
use git2::{build::RepoBuilder, FetchOptions, Sort};
use std::{collections::BTreeMap, path::Path};

pub struct Git {}

impl GitProvider for Git {
    async fn fetch<S: crate::source::DataSource>(
        data_source: S,
        user_name: String,
    ) -> Result<ContributionActivity> {
        let url = "https://github.com/thomas-zahner/commitoria";

        // TODO: is it possible to use the following?
        // --shallow-since "1 year"

        let options = FetchOptions::new();
        let mut builder = RepoBuilder::new();
        builder.fetch_options(options).bare(true);

        let repo = builder
            .clone(url, Path::new("/tmp/cloned-repository"))
            .unwrap();
        let mut revwalk = repo.revwalk().unwrap();

        revwalk.set_sorting(Sort::TIME).unwrap();
        revwalk.push_head().unwrap();

        let one_year_ago = chrono::Local::now()
            .date_naive()
            .checked_sub_months(Months::new(12 * 6)) // todo
            .unwrap();

        let mut result = BTreeMap::new();

        for rev in revwalk.into_iter() {
            let rev = *rev.as_ref().unwrap();
            let commit = repo.find_commit(rev).unwrap();
            let commit_time = DateTime::from_timestamp(commit.time().seconds(), 0)
                .unwrap()
                .date_naive();

            if commit_time >= one_year_ago.try_into().unwrap()
                && (commit.author().name() == Some(&user_name)
                    || commit.author().email() == Some(&user_name))
            {
                *result.entry(commit_time).or_insert(0) += 1;
            }
        }

        Ok(result.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::FixtureDataSource;

    #[tokio::test]
    async fn git_repository() {
        let result = Git::fetch(FixtureDataSource {}, "Thomas Zahner".into())
            .await
            .unwrap();

        todo!("{:?}", result);
    }
}
