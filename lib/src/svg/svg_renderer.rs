use super::contribution_colour::ContributionInfo;
use super::rgba::StringToRgbaError;
use crate::types::ContributionActivity;
use crate::{svg::contribution_colour::ColourStrategy, types::YEAR};
use chrono::{Datelike, Days, NaiveDate, Weekday};

const FONT_SIZE_DEFAULT: usize = 11;
const CELL_SIZE_DEFAULT: usize = 14;
const COLOUR_STRATEGY_DEFAULT: ColourStrategy = ColourStrategy::GitlabStrategy;

#[derive(Default, Debug)]
pub struct Builder {
    pub font_size: Option<usize>,
    pub cell_size: Option<usize>,
    pub colour_strategy: Option<String>,
    pub active_colour: Option<String>,
    pub inactive_colour: Option<String>,
}

impl Builder {
    pub fn build(self) -> Result<SvgRenderer, BuilderError> {
        SvgRenderer::try_from(self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BuilderError {
    UnknownColourStrategy,
    InterpolationParametersMissing,
    InvalidRgbaValue(StringToRgbaError),
}

impl From<StringToRgbaError> for BuilderError {
    fn from(value: StringToRgbaError) -> Self {
        Self::InvalidRgbaValue(value)
    }
}

impl TryFrom<Builder> for SvgRenderer {
    type Error = BuilderError;

    fn try_from(value: Builder) -> Result<Self, Self::Error> {
        let font_size = value.font_size.unwrap_or(FONT_SIZE_DEFAULT);
        let cell_size = value.cell_size.unwrap_or(CELL_SIZE_DEFAULT);
        let colour_strategy = match value.colour_strategy.as_ref().map(|s| s.as_str()) {
            None => COLOUR_STRATEGY_DEFAULT,
            Some("GitlabStrategy") => ColourStrategy::GitlabStrategy,
            Some("InterpolationStrategy") => match (value.inactive_colour, value.active_colour) {
                (Some(inactive), Some(active)) => ColourStrategy::InterpolationStrategy {
                    inactive_colour: inactive.try_into()?,
                    active_colour: active.try_into()?,
                },
                _ => Err(BuilderError::InterpolationParametersMissing)?,
            },
            Some(_) => Err(BuilderError::UnknownColourStrategy)?,
        };

        const DAY_SPACE: usize = 1;
        let day_size_with_space = cell_size + DAY_SPACE * 2;

        Ok(Self {
            font_size,
            cell_size,
            colour_strategy,
            day_size_with_space,
        })
    }
}

pub struct SvgRenderer {
    font_size: usize,
    cell_size: usize,
    colour_strategy: ColourStrategy,
    day_size_with_space: usize,
}

const FIRST_DAY_OF_WEEK: Weekday = Weekday::Mon;
const EXTRA_PADDING: usize = 6;
const MARGIN_HORIZONTAL: usize = 6;

const MONTH_NAMES: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

#[derive(Debug)]
struct Data {
    count: usize,
    date: NaiveDate,
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
        let x = renderer.day_size_with_space * self.group + MARGIN_HORIZONTAL;
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
        let mut result: Vec<Vec<Data>> = vec![]; // todo: functional instead of this weird imperative style
        let mut months: Vec<MonthText> = vec![];
        let mut day = last_day.clone() - YEAR;

        let initial_day = day;

        while day <= last_day {
            if day.weekday() == FIRST_DAY_OF_WEEK || day == initial_day {
                let month = day.month0();
                let is_new_month = match months.last() {
                    None => true,
                    Some(last_month) => month != last_month.month,
                };

                if is_new_month {
                    months.push(MonthText {
                        group: result.len(),
                        month,
                    });
                }

                result.push(vec![]);
            }

            let date = NaiveDate::from_ymd_opt(
                day.year(),
                (day.month() as u8).try_into().unwrap(),
                day.day(),
            )
            .unwrap();

            let count = activity.get(&date).unwrap_or(0);
            let result_index = result.len() - 1;
            result[result_index].push(Data { count, date });

            day = day.checked_add_days(Days::new(1)).unwrap();
        }

        let result_count = result.len();
        let content = self.render_week_rows(result) + "\n" + &self.render_text(months);

        let width = result_count * self.day_size_with_space + MARGIN_HORIZONTAL;
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
        let day_count = result.iter().map(|week| week.len()).sum::<usize>();
        let average_count_per_day = result
            .iter()
            .map(|week| week.iter().map(|day| day.count).sum::<usize>())
            .sum::<usize>() as f32
            / day_count as f32;

        let content = result
            .into_iter()
            .enumerate()
            .map(|(week, day_elements)| {
                let x = self.day_size_with_space * week + MARGIN_HORIZONTAL;
                let y = self.font_size + EXTRA_PADDING;
                let week_day_cells =
                    self.render_week_day_cells(day_elements, average_count_per_day);
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

    fn render_week_day_cells(&self, days: Vec<Data>, average_count_per_day: f32) -> String {
        let cell_size: usize = self.cell_size;
        const CELL_RADIUS: usize = 2;
        const FIST_DAY_OF_WEEK: usize = 0; // todo

        days.into_iter()
            .map(|day| {
                let hover_info = format!("{}", match day.count {
                    0 => "No contributions".to_owned(),
                    1 => "1 contribution".to_owned(),
                    i => format!("{} contributions", i),
                });

                let y = self.day_size_with_space * ((day.date.weekday().num_days_from_monday() as usize + 7 - FIST_DAY_OF_WEEK) % 7);
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
    use chrono::NaiveDate;

    use super::{Builder, SvgRenderer};
    use crate::{provider::github::Github, source::FixtureDataSource, svg::svg_renderer::Data};

    #[tokio::test]
    async fn render_full() {
        let activity = Github::fetch(FixtureDataSource::GithubUser, "".into())
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
                date: NaiveDate::from_ymd_opt(2024, 12.try_into().unwrap(), 2).unwrap(),
            },
            Data {
                count: 0,
                date: NaiveDate::from_ymd_opt(2024, 12.try_into().unwrap(), 3).unwrap(),
            },
            Data {
                count: 1,
                date: NaiveDate::from_ymd_opt(2024, 12.try_into().unwrap(), 4).unwrap(),
            },
            Data {
                count: 2,
                date: NaiveDate::from_ymd_opt(2024, 12.try_into().unwrap(), 5).unwrap(),
            },
            Data {
                count: 17,
                date: NaiveDate::from_ymd_opt(2024, 12.try_into().unwrap(), 6).unwrap(),
            },
            Data {
                count: 0,
                date: NaiveDate::from_ymd_opt(2024, 12.try_into().unwrap(), 7).unwrap(),
            },
            Data {
                count: 0,
                date: NaiveDate::from_ymd_opt(2024, 12.try_into().unwrap(), 8).unwrap(),
            },
        ]];

        let svg = get_renderer().render_week_rows(data);
        let fixture = read_fixture("fixtures/week_group.svg");
        assert_eq!(svg, fixture.trim());
    }

    fn get_renderer() -> SvgRenderer {
        Builder::default().build().unwrap()
    }

    fn read_fixture(path: &str) -> String {
        std::fs::read_to_string(path).expect("Unable to read file")
    }
}
