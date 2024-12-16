use crate::{
    provider::{parse_date, Error},
    source::{DataSource, Source},
    ContributionActivity,
};
use std::collections::HashMap;
use time::Date;

use super::{GitProvider, Result};

pub struct Gitlab {}

impl GitProvider for Gitlab {
    fn fetch<S: DataSource>(data_source: S, user_name: String) -> Result<ContributionActivity> {
        let json = data_source.fetch(Source::GitlabUser(user_name));
        let parsed: HashMap<String, usize> =
            serde_json::from_str(&json).map_err(|e| Error::UnableToParseJson(e.to_string()))?;

        Ok(ContributionActivity(
            parsed
                .into_iter()
                .map(|(date, contribution_count)| -> Result<(Date, usize)> {
                    Ok((parse_date(&date)?, contribution_count))
                })
                .collect::<Result<HashMap<_, _>>>()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::LocalDataSource;

    #[test]
    fn gitlab_contributions() {
        let result = Gitlab::fetch(LocalDataSource {}, "".into()).unwrap();

        assert_eq!(
            result.get(&Date::from_calendar_date(2024, time::Month::January, 22).unwrap()),
            Some(1)
        );

        assert_eq!(
            result.get(&Date::from_calendar_date(2024, time::Month::February, 4).unwrap()),
            Some(2)
        );

        assert_eq!(
            result.get(&Date::from_calendar_date(2024, time::Month::January, 1).unwrap()),
            None
        );
    }

    #[test]
    fn gitlab_contribution_sum() {
        let result = Gitlab::fetch(LocalDataSource {}, "".into()).unwrap();
        assert_eq!(result.contribution_count(), 21);
    }
}
