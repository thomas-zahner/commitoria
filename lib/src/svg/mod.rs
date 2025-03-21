use crate::svg::contribution_colour::ColourStrategy;
use crate::types::ContributionActivity;
use chrono::{Datelike, Days, Months, NaiveDate, Weekday};
use contribution_colour::ContributionInfo;
use derive_builder::Builder;
use time::Date;

pub mod contribution_colour;
pub mod rgba;

const FONT_SIZE_DEFAULT: usize = 11;
const CELL_SIZE_DEFAULT: usize = 14;
const COLOUR_STRATEGY_DEFAULT: ColourStrategy = ColourStrategy::GitlabStrategy;

#[derive(Builder)]
pub struct SvgRenderer {
    #[builder(default = "FONT_SIZE_DEFAULT")]
    font_size: usize,

    #[builder(default = "CELL_SIZE_DEFAULT")]
    cell_size: usize,

    #[builder(default = "COLOUR_STRATEGY_DEFAULT")]
    colour_strategy: ColourStrategy,

    #[builder(default = "self.day_size_with_space()")]
    #[builder(setter(skip))]
    day_size_with_space: usize,
}

impl SvgRendererBuilder {
    fn day_size_with_space(&self) -> usize {
        const DAY_SPACE: usize = 1;
        self.cell_size.unwrap_or(CELL_SIZE_DEFAULT) + DAY_SPACE * 2
    }
}

const FIRST_DAY_OF_WEEK: Weekday = Weekday::Mon;
const EXTRA_PADDING: usize = 6;

const MONTH_NAMES: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

#[derive(Debug)]
struct Data {
    count: usize,
    date: Date,
}

struct MonthText {
    group: usize,
    month: u32,
}

impl MonthText {
    fn get_text(&self) -> String {
        MONTH_NAMES[self.month as usize].to_owned()
    }

    fn render(&self, renderer: &SvgRenderer) -> String {
        let x = renderer.day_size_with_space * self.group + 1 + renderer.day_size_with_space;
        let y = renderer.font_size;

        format!(
            r#"<text x="{}" y="{}" class="user-contrib-text">{}</text>"#,
            x,
            y,
            self.get_text()
        )
    }
}

impl SvgRenderer {
    pub fn render(&self, activity: &ContributionActivity) -> String {
        let today = chrono::Local::now().date_naive();
        self.render_at(activity, today)
    }

    fn render_at(&self, activity: &ContributionActivity, last_day: NaiveDate) -> String {
        let mut group = 0;
        let mut result: Vec<Vec<Data>> = vec![vec![]]; // todo: functional instead of this weird imperative style
        let mut months: Vec<MonthText> = vec![];

        let mut day = last_day
            .clone()
            .checked_sub_months(Months::new(12))
            .unwrap();

        while day <= last_day {
            if day.weekday() == FIRST_DAY_OF_WEEK {
                group += 1;
                result.push(vec![]);

                let month = day.month0();
                let is_new_month = match months.last() {
                    None => true,
                    Some(last_month) => month != last_month.month,
                };

                if is_new_month {
                    months.push(MonthText { group, month });
                }
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

        let content = self.render_week_rows(result) + "\n" + &self.render_text(months);

        let width = (group + 2) * self.day_size_with_space + EXTRA_PADDING;
        let height = self.font_size + 7 * self.day_size_with_space + EXTRA_PADDING;
        self.wrap_svg(width, height, &content)
    }

    fn wrap_svg(&self, width: usize, height: usize, content: &str) -> String {
        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" class="contrib-calendar" data-testid="contrib-calendar">
    {}
    {}
</svg>"#,
            width,
            height,
            self.get_style(),
            content
        )
    }

    fn render_week_rows(&self, result: Vec<Vec<Data>>) -> String {
        let content = result
            .into_iter()
            .enumerate()
            .map(|(week, day_elements)| {
                let x = self.day_size_with_space * week + 1 + self.day_size_with_space;
                let y = self.font_size + EXTRA_PADDING;
                let week_day_cells = self.render_week_day_cells(day_elements);
                format!(
                    r#"<g transform="translate({}, {})" data-testid="user-contrib-cell-group">
{}
</g>"#,
                    x, y, week_day_cells
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        content
    }

    fn render_week_day_cells(&self, days: Vec<Data>) -> String {
        let cell_size: usize = self.cell_size;
        const CELL_RADIUS: usize = 2;
        const FIST_DAY_OF_WEEK: usize = 0; // todo

        let average_count_per_day =
            days.iter().map(|day| day.count).sum::<usize>() as f32 / days.len() as f32;

        days.into_iter()
            .map(|day| {
                let hover_info = format!("{}", match day.count {
                    0 => "No contributions".to_owned(),
                    1 => "1 contribution".to_owned(),
                    i => format!("{} contributions", i),
                });
                let y = self.day_size_with_space * ((day.date.weekday().number_days_from_monday() as usize + 7 - FIST_DAY_OF_WEEK) % 7);
                let data_date = day.date.to_string();
                let colour = self.colour_strategy.get_colour(ContributionInfo {
                    average_count_per_day ,
                    count_today: day.count,
                });

                format!(r#"<rect x="0" y="{y}" rx="{CELL_RADIUS}" ry="{CELL_RADIUS}" width="{cell_size}" height="{cell_size}" fill="{colour}" data-hover-info="{hover_info}" data-date="{data_date}" class="user-contrib-cell has-tooltip"></rect>"#)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn render_text(&self, months: Vec<MonthText>) -> String {
        format!(
            r#"<g direction="ltr">{}</g>"#,
            months
                .iter()
                .map(|month| month.render(&self))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    fn get_style(&self) -> String {
        format!(
            r#"<style>
            :root {{
                --text-color-default: #3a383f;
                --border-color-default: #dcdcde;
            }}

            .user-contrib-text {{
                font-size: {}px;
                font-family: "Noto Sans", Ubuntu, Cantarell, "Helvetica Neue", sans-serif;
            }}
        </style>"#,
            self.font_size
        )
    }
}

#[cfg(test)]
mod tests {
    use time::Date;

    use super::{SvgRenderer, SvgRendererBuilder};
    use crate::{
        provider::{github::Github, GitProvider},
        source::FixtureDataSource,
        svg::Data,
    };

    #[tokio::test]
    async fn render_full() {
        let activity = Github::fetch(FixtureDataSource {}, "".into())
            .await
            .unwrap();

        let today = chrono::naive::NaiveDate::from_ymd_opt(2024, 12, 13).unwrap();

        let svg = get_renderer().render_at(&activity, today);
        let fixture = read_fixture("fixtures/activity.svg");
        assert_eq!(svg, fixture.trim());
    }

    #[test]
    fn render_week() {
        let data = vec![vec![
            Data {
                count: 0,
                date: Date::from_calendar_date(2024, 12.try_into().unwrap(), 2).unwrap(),
            },
            Data {
                count: 0,
                date: Date::from_calendar_date(2024, 12.try_into().unwrap(), 3).unwrap(),
            },
            Data {
                count: 1,
                date: Date::from_calendar_date(2024, 12.try_into().unwrap(), 4).unwrap(),
            },
            Data {
                count: 2,
                date: Date::from_calendar_date(2024, 12.try_into().unwrap(), 5).unwrap(),
            },
            Data {
                count: 17,
                date: Date::from_calendar_date(2024, 12.try_into().unwrap(), 6).unwrap(),
            },
            Data {
                count: 0,
                date: Date::from_calendar_date(2024, 12.try_into().unwrap(), 7).unwrap(),
            },
            Data {
                count: 0,
                date: Date::from_calendar_date(2024, 12.try_into().unwrap(), 8).unwrap(),
            },
        ]];

        let svg = get_renderer().render_week_rows(data);
        let fixture = read_fixture("fixtures/week_group.svg");
        assert_eq!(svg, fixture.trim());
    }

    fn get_renderer() -> SvgRenderer {
        SvgRendererBuilder::default().build().unwrap()
    }

    fn read_fixture(path: &str) -> String {
        std::fs::read_to_string(path).expect("Unable to read file")
    }
}
