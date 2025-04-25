use crate::{
    provider::parse_date,
    source::{DataSource, Source},
    types::ContributionActivity,
};
use chrono::NaiveDate;
use std::collections::{BTreeMap, HashMap};

use super::{GitProvider, Result};

pub struct Gitea {}

impl GitProvider for Gitea {
    async fn fetch<S: DataSource>(
        data_source: S,
        user_name: String,
    ) -> Result<ContributionActivity> {
        todo!()

        // URL: https://codeberg.org/unfa?tab=activity
        //
        // Select div#user-heatmap
        // Has attribute `data-heatmap-data`
        //
        // Attribute value:
        //
        // [{&#34;timestamp&#34;:1720530900,&#34;contributions&#34;:2},{&#34;timestamp&#34;:1720531800,&#34;contributions&#34;:4},{&#34;timestamp&#34;:1722285000,&#34;contributions&#34;:1},{&#34;timestamp&#34;:1729594800,&#34;contributions&#34;:1},{&#34;timestamp&#34;:1729607400,&#34;contributions&#34;:1},{&#34;timestamp&#34;:1732624200,&#34;contributions&#34;:2},{&#34;timestamp&#34;:1733067900,&#34;contributions&#34;:1},{&#34;timestamp&#34;:1741988700,&#34;contributions&#34;:3},{&#34;timestamp&#34;:1741989600,&#34;contributions&#34;:1},{&#34;timestamp&#34;:1741990500,&#34;contributions&#34;:2},{&#34;timestamp&#34;:1741991400,&#34;contributions&#34;:2},{&#34;timestamp&#34;:1744729200,&#34;contributions&#34;:1},{&#34;timestamp&#34;:1745508600,&#34;contributions&#34;:2}]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        source::{FixtureDataSource, ReqwestDataSource},
        types::Error,
    };

    #[tokio::test]
    async fn contributions_fixture() {
        let result = Gitea::fetch(FixtureDataSource {}, "".into()).await.unwrap();

        assert_eq!(
            result.get(&NaiveDate::from_ymd_opt(2024, 01, 22).unwrap()),
            Some(1)
        );

        assert_eq!(
            result.get(&NaiveDate::from_ymd_opt(2024, 02, 04).unwrap()),
            Some(2)
        );

        assert_eq!(
            result.get(&NaiveDate::from_ymd_opt(2024, 01, 01).unwrap()),
            None
        );
        assert_eq!(result.contribution_count(), 21);
    }

    #[tokio::test]
    async fn contributions_real() {
        let result = Gitea::fetch(ReqwestDataSource {}, "unfa".into()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn user_not_found() {
        let result = Gitea::fetch(ReqwestDataSource {}, "".into()).await;
        assert_eq!(result, Result::Err(Error::UserNotFound));
    }
}
