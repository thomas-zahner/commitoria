use chrono::NaiveDate;
use std::{
    collections::BTreeMap,
    ops::{Add, AddAssign},
};

#[cfg(feature = "serde")]
use serde::{ser::SerializeMap, Serialize, Serializer};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ContributionActivity(BTreeMap<NaiveDate, usize>);

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
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn get(&self, date: &NaiveDate) -> Option<usize> {
        self.0.get(date).map(|c| c.clone())
    }

    pub fn active_days(&self) -> usize {
        self.0.len()
    }

    pub fn contribution_count(&self) -> usize {
        self.0.iter().map(|(_, count)| count).sum()
    }
}

impl From<BTreeMap<NaiveDate, usize>> for ContributionActivity {
    fn from(value: BTreeMap<NaiveDate, usize>) -> Self {
        Self(value)
    }
}

impl Add for ContributionActivity {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for (k, v) in rhs.0.into_iter() {
            self.0
                .entry(k)
                .and_modify(|e| {
                    *e += v;
                })
                .or_insert(v);
        }

        self
    }
}

impl AddAssign for ContributionActivity {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::ContributionActivity;
    use chrono::NaiveDate;
    use std::collections::BTreeMap;

    #[test]
    fn aggregate() {
        let first = NaiveDate::from_ymd_opt(2024, 01, 01).unwrap();
        let second = NaiveDate::from_ymd_opt(2024, 01, 02).unwrap();
        let activity = ContributionActivity(BTreeMap::from([(first, 1), (second, 2)]))
            + ContributionActivity(BTreeMap::from([(first, 3)]));

        assert_eq!(activity.get(&first), Some(4));
        assert_eq!(activity.get(&second), Some(2));
        let third = NaiveDate::from_ymd_opt(2024, 01, 03).unwrap();
        assert_eq!(activity.get(&third), None);
    }
}
