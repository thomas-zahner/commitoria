use fetch::{DataSource, Source};
use regex::Regex;
use scraper::{error::SelectorErrorKind, Html, Selector};
use time::{format_description::BorrowedFormatItem, macros::format_description, Date};

mod fetch;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(PartialEq, Eq, Debug)]
pub struct ContributionActivity {
    activities: Vec<DailyActivity>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct DailyActivity {
    contribution_count: usize,
    date: Date,
}

#[derive(Debug)]
pub enum Error {
    SelectorError(String),
    AttributeMissing,
    TooltipMissing,
    UnexpectedTooltipMessage(String),
}

impl From<SelectorErrorKind<'_>> for Error {
    fn from(value: SelectorErrorKind<'_>) -> Self {
        Self::SelectorError(value.to_string())
    }
}

pub trait GitProvider {
    fn fetch<S: DataSource>(data_source: S, user_name: String) -> Result<ContributionActivity>;
}

pub struct Github {}

const GITHUB_DATE_DESCRIPTION: &'static [BorrowedFormatItem<'static>] =
    format_description!("[year]-[month]-[day]");

impl GitProvider for Github {
    fn fetch<S: DataSource>(data_source: S, user_name: String) -> Result<ContributionActivity> {
        let html = data_source.fetch(Source::GithubUser(user_name));
        let document = Html::parse_document(&html);
        let selector = Selector::parse("div > table > tbody td[data-date]")?;

        let activities: Vec<DailyActivity> = document
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
                    Regex::new("^(\\d+) contributions?")
                        .unwrap()
                        .captures(&tool_tip_text)
                        .ok_or(Error::UnexpectedTooltipMessage(tool_tip_text.clone()))?
                        .get(1)
                        .ok_or(Error::UnexpectedTooltipMessage(tool_tip_text.clone()))?
                        .as_str()
                        .parse()
                        .map_err(|_| Error::UnexpectedTooltipMessage(tool_tip_text))?
                };

                Ok(DailyActivity {
                    date: Date::parse(date, GITHUB_DATE_DESCRIPTION).unwrap(),
                    contribution_count,
                })
            })
            .collect::<Result<_>>()?;

        Ok(ContributionActivity { activities })
    }
}

#[cfg(test)]
mod tests {
    use fetch::LocalDataSource;

    use super::*;

    #[test]
    fn github_contributions() {
        let result = Github::fetch(LocalDataSource {}, "".into()).unwrap();

        assert_eq!(result.activities.len(), 370);
        assert_eq!(
            result.activities[0],
            DailyActivity {
                contribution_count: 0,
                date: Date::from_calendar_date(2023, time::Month::December, 10).unwrap(),
            }
        );
        assert_eq!(
            result.activities[23],
            DailyActivity {
                contribution_count: 1,
                date: Date::from_calendar_date(2024, time::Month::May, 19).unwrap(),
            }
        );
    }

    #[test]
    fn github_contribution_sum() {
        let result = Github::fetch(LocalDataSource {}, "".into()).unwrap();
        let sum: usize = result
            .activities
            .into_iter()
            .map(|a| a.contribution_count)
            .sum();
        assert_eq!(sum, 191);
    }
}
