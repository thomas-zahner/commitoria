use fetch::{DataSource, Source};
use regex::Regex;
use scraper::{error::SelectorErrorKind, Html, Selector};

mod fetch;

#[derive(PartialEq, Eq, Debug)]
pub struct ContributionActivity {
    activities: Vec<DailyActivity>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct DailyActivity {
    contribution_count: usize,
    date: String, // todo: use chrono::Date
}

#[derive(Debug)]
pub enum Error {
    SelectorError(String),
    AttributeMissing,
    TooltipMissing,
}

impl From<SelectorErrorKind<'_>> for Error {
    fn from(value: SelectorErrorKind<'_>) -> Self {
        Self::SelectorError(value.to_string())
    }
}

trait GitProvider {
    fn fetch<S: DataSource>(
        data_source: S,
        user_name: String,
    ) -> Result<ContributionActivity, Error>;
}

pub struct Github {}

impl GitProvider for Github {
    fn fetch<S: DataSource>(
        data_source: S,
        user_name: String,
    ) -> Result<ContributionActivity, Error> {
        let html = data_source.fetch(Source::GithubUser(user_name));
        let document = Html::parse_document(&html);
        let selector = Selector::parse("div > table > tbody td[data-date]")?;

        let x: Vec<DailyActivity> = document
            .select(&selector)
            .map(|element| -> Result<DailyActivity, Error> {
                let date = element.attr("data-date").ok_or(Error::AttributeMissing)?;
                let id = element.attr("id").ok_or(Error::AttributeMissing)?;

                let selector = Selector::parse(&format!(r#"tool-tip[for="{}"]"#, id))?;
                let tool_tip = document
                    .select(&selector)
                    .next()
                    .ok_or(Error::TooltipMissing)?;

                dbg!(date, tool_tip.inner_html());
                let contribution_count = 0; // todo: regex?

                Ok(DailyActivity {
                    date: date.into(),
                    contribution_count,
                })
            })
            .collect::<Result<Vec<DailyActivity>, Error>>()?;

        Ok(ContributionActivity { activities: x })
    }
}

#[cfg(test)]
mod tests {
    use fetch::LocalDataSource;

    use super::*;

    #[test]
    fn github() {
        let source = LocalDataSource {};
        let result = Github::fetch(source, "foo".into()).unwrap();
        assert_eq!(result, ContributionActivity { activities: vec![] });
    }
}
