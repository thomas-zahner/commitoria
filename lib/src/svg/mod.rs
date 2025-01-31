use chrono::{DateTime, Datelike, Days, Local, Months, TimeZone, Utc, Weekday};
use time::{Date, Month, Time};

use crate::types::ContributionActivity;

pub struct SvgRenderer {}

const STYLE: &str = r#"<style>
    :root {
        --user-activity-0: #ececef;
        --user-activity-1: #d2dcff;
        --user-activity-2: #7992f5;
        --user-activity-3: #4e65cd;
        --user-activity-4: #303470;
        --text-color-default: #3a383f;
        --border-color-default: #dcdcde;
    }

    .user-contrib-text {
        font-size: 11px;
    }

    .user-contrib-cell[data-level="0"] {
        fill: var(--user-activity-0);
    }
    .user-contrib-cell[data-level="1"] {
        fill: var(--user-activity-1);
    }
    .user-contrib-cell[data-level="2"] {
        fill: var(--user-activity-2);
    }
    .user-contrib-cell[data-level="3"] {
        fill: var(--user-activity-3);
    }
    .user-contrib-cell[data-level="4"] {
        fill: var(--user-activity-4);
    }
</style>"#;

const DAY_SPACE: usize = 1;
const DAY_SIZE: usize = 14;
const DAY_SIZE_WITH_SPACE: usize = DAY_SIZE + DAY_SPACE * 2;
const FIRST_DAY_OF_WEEK: Weekday = Weekday::Mon;

const MONTH_NAMES: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

#[derive(Debug)]
struct Data {
    count: usize,
    date: Date,
}

impl SvgRenderer {
    pub fn render<T: TimeZone>(activity: &ContributionActivity, last_day: DateTime<T>) -> String {
        let mut group = 0;
        let mut result: Vec<Vec<Data>> = vec![vec![]]; // todo: functional instead of this weird imperative style

        let mut day = last_day
            .clone()
            .checked_sub_months(Months::new(12))
            .unwrap();

        while day < last_day {
            if day.weekday() == FIRST_DAY_OF_WEEK && group != 0 {
                group += 1;
                result.push(vec![]);
            }

            let date = Date::from_calendar_date(
                day.year(),
                (day.month() as u8).try_into().unwrap(),
                day.day() as u8,
            )
            .unwrap();

            let count = activity.get(&date).unwrap_or(0);
            result[group].push(Data { count, date });

            day = day.checked_add_days(Days::new(1)).unwrap();
        }

        dbg!(result);
        Self::wrap_svg(activity)
    }

    fn wrap_svg(activity: &ContributionActivity) -> String {
        const WIDTH: u16 = 864; // TODO: use group value to calculate
        const HEIGHT: u16 = 140;

        format!(
            r#"<svg width="{}" height="{}" class="contrib-calendar" data-testid="contrib-calendar">
    {}
</svg>"#,
            WIDTH, HEIGHT, STYLE
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        provider::gitlab::Gitlab, provider::GitProvider, source::FixtureDataSource, svg::STYLE,
    };

    use super::SvgRenderer;

    #[tokio::test]
    async fn basic() {
        let activity = Gitlab::fetch(FixtureDataSource {}, "".into())
            .await
            .unwrap();

        let today = chrono::offset::Local::now();
        let svg = SvgRenderer::render(&activity, today);
        assert_eq!(
            &svg,
            &format!(
                r#"<svg width="864" height="140" class="contrib-calendar" data-testid="contrib-calendar">
    {}
</svg>"#,
                STYLE
            )
        )
    }
}
