use scraper::error::SelectorErrorKind;
use std::collections::HashMap;
use time::Date;

#[cfg(feature = "serde")]
use serde::{ser::SerializeMap, Serialize, Serializer};

pub mod provider;
pub mod source;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SelectorError(String),
    AttributeMissing,
    TooltipMissing,
    UnexpectedTooltipMessage(String),
    UnableToParseDate(String),
    UnableToParseJson(String),
    ReqwestError(String),
    UserNotFound,
}

impl From<SelectorErrorKind<'_>> for Error {
    fn from(value: SelectorErrorKind<'_>) -> Self {
        Self::SelectorError(value.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value.to_string())
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ContributionActivity(HashMap<Date, usize>);

#[cfg(feature = "serde")]
impl Serialize for ContributionActivity {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (k, v) in &self.0 {
            map.serialize_entry(&k.to_string(), &v)?;
        }
        map.end()
    }
}

impl ContributionActivity {
    pub fn get(&self, date: &Date) -> Option<usize> {
        self.0.get(date).map(|c| c.clone())
    }

    pub fn active_days(&self) -> usize {
        self.0.len()
    }

    pub fn contribution_count(&self) -> usize {
        self.0.iter().map(|(_, count)| count).sum()
    }

    pub fn combine(&mut self, other: Self) {
        for (k, v) in other.0.into_iter() {
            self.0
                .entry(k)
                .and_modify(|e| {
                    *e += v;
                })
                .or_insert(v);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ContributionActivity;
    use std::collections::HashMap;
    use time::Date;

    #[test]
    fn aggregate() {
        let first = Date::from_calendar_date(2024, time::Month::January, 1).unwrap();
        let second = Date::from_calendar_date(2024, time::Month::January, 2).unwrap();
        let mut activity = ContributionActivity(HashMap::from([(first, 1), (second, 2)]));

        activity.combine(ContributionActivity(HashMap::from([(first, 3)])));

        assert_eq!(activity.get(&first), Some(4));
        assert_eq!(activity.get(&second), Some(2));
        let third = Date::from_calendar_date(2024, time::Month::January, 3).unwrap();
        assert_eq!(activity.get(&third), None);
    }
}
