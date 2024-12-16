use std::collections::HashMap;
use time::Date;

pub mod provider;
pub mod source;

#[derive(PartialEq, Eq, Debug)]
pub struct ContributionActivity(HashMap<Date, usize>);

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
        let mut combined = ContributionActivity(HashMap::from([(first, 1), (second, 2)]));

        combined.combine(ContributionActivity(HashMap::from([(first, 3)])));

        assert_eq!(combined.get(&first), Some(4));
        assert_eq!(combined.get(&second), Some(2));
        let third = Date::from_calendar_date(2024, time::Month::January, 3).unwrap();
        assert_eq!(combined.get(&third), None);
    }
}
