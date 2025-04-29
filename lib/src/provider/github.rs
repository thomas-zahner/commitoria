use super::{parse_date, Error};
use crate::{source::DataSource, types::ContributionActivity, types::Result};
use regex::Regex;
use scraper::{Html, Selector};
use std::{collections::BTreeMap, sync::LazyLock};

pub struct Github {}

static GITHUB_CONTRIBUTION_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^(\\d+) contributions?").unwrap());

impl Github {
    pub async fn fetch<S: DataSource>(
        data_source: S,
        user_name: String,
    ) -> Result<ContributionActivity> {
        let html = data_source
            .fetch(format!(
                "https://github.com/users/{user_name}/contributions"
            ))
            .await?;
        let document = Html::parse_document(&html);
        let selector = Selector::parse("div > table > tbody td[data-date]")?;

        let activities = document
            .select(&selector)
            .map(|element| {
                let date = element.attr("data-date").ok_or(Error::AttributeMissing)?;
                let id = element.attr("id").ok_or(Error::AttributeMissing)?;

                let selector = Selector::parse(&format!(r#"tool-tip[for="{}"]"#, id))?;
                let tool_tip_text = document
                    .select(&selector)
                    .next()
                    .ok_or(Error::TooltipMissing)?
                    .inner_html();

                let contribution_count = if tool_tip_text.starts_with("No contributions") {
                    0
                } else {
                    GITHUB_CONTRIBUTION_REGEX
                        .captures(&tool_tip_text)
                        .ok_or(Error::UnexpectedTooltipMessage(tool_tip_text.clone()))?
                        .get(1)
                        .ok_or(Error::UnexpectedTooltipMessage(tool_tip_text.clone()))?
                        .as_str()
                        .parse()
                        .map_err(|_| Error::UnexpectedTooltipMessage(tool_tip_text))?
                };

                Ok((parse_date(date)?, contribution_count))
            })
            .collect::<Result<BTreeMap<_, _>>>()?;

        Ok(activities.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::{FixtureDataSource, ReqwestDataSource};
    use chrono::NaiveDate;

    #[tokio::test]
    async fn contributions_fixture() {
        let result = Github::fetch(FixtureDataSource::GithubUser, "".into())
            .await
            .unwrap();

        assert_eq!(result.active_days(), 370);
        assert_eq!(
            result.get(&NaiveDate::from_ymd_opt(2023, 12, 10).unwrap()),
            Some(0)
        );
        assert_eq!(
            result.get(&NaiveDate::from_ymd_opt(2024, 05, 19).unwrap()),
            Some(1)
        );
        assert_eq!(result.contribution_count(), 191);
    }

    #[tokio::test]
    async fn contributions_real() {
        let result = Github::fetch(ReqwestDataSource {}, "mre".into()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn user_not_found() {
        let result = Github::fetch(ReqwestDataSource {}, "".into()).await;
        assert_eq!(result, Result::Err(Error::UserNotFound));
    }
}
