use std::collections::HashMap;

use chrono::{DateTime, Datelike, Days, Months, TimeZone, Weekday};
use time::Date;

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
const SVG_HEIGHT: usize = 140;

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
            if day.weekday() == FIRST_DAY_OF_WEEK {
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

        let content = Self::render_month_rows(result);

        let width = (group + 1) * DAY_SIZE_WITH_SPACE; // TODO: handle extra padding case (getExtraWidthPadding)
        Self::wrap_svg(width, &content)
    }

    fn wrap_svg(width: usize, content: &str) -> String {
        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" class="contrib-calendar" data-testid="contrib-calendar">
    {}
    {}
</svg>"#,
            width, SVG_HEIGHT, STYLE, content
        )
    }

    fn render_month_rows(result: Vec<Vec<Data>>) -> String {
        let content = result.into_iter().enumerate().map(|(week, day_elements)| {
            let x = DAY_SIZE_WITH_SPACE * week + 1 + DAY_SIZE_WITH_SPACE;
            let week_day_cells = Self::render_week_day_cells(day_elements);
            dbg!(format!(
                r#"<g transform="translate({}, 18)" data-testid="user-contrib-cell-group">{}</g>"#,
                x,
                week_day_cells
            ))
        }).collect::<Vec<_>>().join("\n");
        content
    }

    fn render_week_day_cells(days: Vec<Data>) -> String {
        const CELL_SIZE: usize = 14;
        const CELL_RADIUS: usize = 2;
        const FIST_DAY_OF_WEEK: usize = 0; // todo

        days.into_iter()
            .map(|day| {
                let (data_level, hover_info )  = Self::get_contrib_level(day.count);
                let y = DAY_SIZE_WITH_SPACE * ((day.date.weekday().number_days_from_monday() as usize + 7 - FIST_DAY_OF_WEEK) % 7);

                format!(r#"<rect x="0" y="{y}" rx="{CELL_RADIUS}" ry="{CELL_RADIUS}" width="{CELL_SIZE}" height="{CELL_SIZE}" data-level="{data_level}" data-hover-info="{hover_info}" class="user-contrib-cell has-tooltip" data-testid="user-contrib-cell" data-html="true" data-container="body"></rect>"#)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn get_contrib_level(count: usize) -> (usize, String) {
        let contrib_levels = HashMap::from([
            (30, (4, "30+ contributions".into())),
            (20, (3, "20-29 contributions".into())),
            (10, (2, "10-19 contributions".into())),
            (1, (1, "1-9 contributions".into())),
            (0usize, (0, "No contributions".into())),
        ]);

        contrib_levels
            .into_iter()
            .find(|(min, _)| count >= *min)
            .unwrap()
            .1
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{
        provider::{github::Github, GitProvider},
        source::FixtureDataSource,
        svg::STYLE,
    };

    use super::SvgRenderer;

    #[tokio::test]
    async fn basic() {
        let activity = Github::fetch(FixtureDataSource {}, "".into())
            .await
            .unwrap();

        let today = chrono::offset::Local::now();
        let svg = SvgRenderer::render(&activity, today);

        fs::write("/tmp/activity.svg", &svg).unwrap(); // todo: remove

        assert_eq!(
            &svg,
            &format!(
                r#"<svg width="848" height="140" class="contrib-calendar" data-testid="contrib-calendar">
    {}
</svg>"#,
                STYLE
            )
        )
    }
}
