use crate::{provider::parse_date, source::DataSource, types::ContributionActivity};
use chrono::NaiveDate;
use std::collections::{BTreeMap, HashMap};
use url::Url;

use super::Result;

pub struct Gitlab {}

impl Gitlab {
    pub async fn fetch<S: DataSource>(
        data_source: S,
        user_name: String,
        mut url: Url,
    ) -> Result<ContributionActivity> {
        url.set_path(&format!("users/{user_name}/calendar.json"));
        let json = data_source.fetch(url).await?;
        let parsed: HashMap<String, usize> = serde_json::from_str(&json)?;

        Ok(parsed
            .into_iter()
            .map(|(date, contribution_count)| -> Result<(NaiveDate, usize)> {
                Ok((parse_date(&date)?, contribution_count))
            })
            .collect::<Result<BTreeMap<_, _>>>()?
            .into())
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
        let result = Gitlab::fetch(
            FixtureDataSource::GitlabUser,
            "".into(),
            "https://gitlab.com".try_into().unwrap(),
        )
        .await
        .unwrap();

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
        let result = Gitlab::fetch(
            ReqwestDataSource {},
            "thomas-zahner".into(),
            "https://gitlab.com".try_into().unwrap(),
        )
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn user_not_found() {
        let result = Gitlab::fetch(
            ReqwestDataSource {},
            "".into(),
            "https://gitlab.com".try_into().unwrap(),
        )
        .await;
        assert_eq!(result, Result::Err(Error::UserNotFound));
    }
}
