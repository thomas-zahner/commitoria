use super::{GitProvider, Result};
use crate::types::{ContributionActivity, YEAR};
use chrono::DateTime;
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

        let repo = builder.clone(url, Path::new("/tmp/cloned-repository"))?;
        let mut revwalk = repo.revwalk()?;

        revwalk.set_sorting(Sort::TIME)?;
        revwalk.push_head()?;

        let mut result = BTreeMap::new();
        let one_year_ago = chrono::Local::now().date_naive() - YEAR;

        for rev in revwalk.into_iter() {
            let rev = *rev.as_ref()?;
            let commit = repo.find_commit(rev)?;
            let commit_time = DateTime::from_timestamp(commit.time().seconds(), 0)
                .ok_or(super::Error::UnableToParseDate(
                    "Invalid timestamp encountered".into(),
                ))?
                .date_naive();

            if commit_time >= one_year_ago
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
