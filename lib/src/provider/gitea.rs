use crate::{
    source::DataSource,
    types::{ContributionActivity, Error},
};
use chrono::{DateTime, NaiveDate};
use scraper::{Html, Selector};
use serde::Deserialize;
use std::collections::BTreeMap;

use super::Result;

pub struct Gitea {}

#[derive(Deserialize, Debug)]
struct HeatmapDataPoint {
    timestamp: i64,
    contributions: usize,
}

impl TryFrom<Vec<HeatmapDataPoint>> for ContributionActivity {
    type Error = Error;

    fn try_from(value: Vec<HeatmapDataPoint>) -> std::result::Result<Self, Self::Error> {
        let data_points = value
            .into_iter()
            .map(|data_point| {
                let timestamp = DateTime::from_timestamp(data_point.timestamp, 0).ok_or(
                    Error::UnableToParseDate("Invalid timestamp encountered".into()),
                );
                Ok((timestamp?.date_naive(), data_point.contributions))
            })
            .collect::<Result<Vec<(NaiveDate, usize)>>>()?;

        let mut map = BTreeMap::new();
        for (timestamp, count) in data_points {
            map.entry(timestamp)
                .and_modify(|v| *v += count)
                .or_insert(count);
        }

        Ok(map.into())
    }
}

impl Gitea {
    pub async fn fetch<S: DataSource>(
        data_source: S,
        user_name: String,
    ) -> Result<ContributionActivity> {
        let hostname = "codeberg.org".to_string();
        let html = data_source
            .fetch(format!("https://{hostname}/{user_name}?tab=activity"))
            .await?;

        let document = Html::parse_document(&html);
        let selector = Selector::parse("div#user-heatmap")?;

        let selected = document
            .select(&selector)
            .next()
            .ok_or(Error::UserNotFound)?;
        let json = selected
            .attr("data-heatmap-data")
            .ok_or(Error::AttributeMissing)?;

        let parsed: Vec<HeatmapDataPoint> = serde_json::from_str(&json)?;
        parsed.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::{FixtureDataSource, ReqwestDataSource};

    #[tokio::test]
    async fn contributions_fixture() {
        let result = Gitea::fetch(FixtureDataSource::GiteaUser, "".into())
            .await
            .unwrap();

        assert_eq!(
            result.get(&NaiveDate::from_ymd_opt(2024, 07, 09).unwrap()),
            Some(6)
        );

        assert_eq!(
            result.get(&NaiveDate::from_ymd_opt(2024, 07, 29).unwrap()),
            Some(1)
        );

        assert_eq!(result.contribution_count(), 23);
    }

    #[tokio::test]
    async fn contributions_real() {
        let result = Gitea::fetch(ReqwestDataSource {}, "unfa".into()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn contributions_real_forgejo() {
        // url: git.omaps.dev
        // user: kirylkaveryn
        todo!()
    }

    #[tokio::test]
    async fn user_not_found() {
        let result = Gitea::fetch(ReqwestDataSource {}, "".into()).await;
        assert_eq!(result, Result::Err(Error::UserNotFound));
    }
}
