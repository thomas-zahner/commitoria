use commitoria_lib::{provider::RepositoryInfo, svg::svg_renderer};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub(crate) struct CalendarQuery {
    pub(crate) github: Option<String>,
    pub(crate) gitlab: Option<String>,
    font_size: Option<usize>,
    cell_size: Option<usize>,
    colour_strategy: Option<String>,
    active_colour: Option<String>,
    inactive_colour: Option<String>,
    bare: Option<Vec<String>>,
    gitea: Option<Vec<String>>,
}

pub(crate) struct Repositories {
    pub(crate) gitlab: Option<String>,
    pub(crate) github: Option<String>,
    pub(crate) gitea: Vec<RepositoryInfo>,
    pub(crate) bare: Vec<RepositoryInfo>,
}

pub(crate) struct ParsedQuery(pub(crate) Repositories, pub(crate) svg_renderer::Builder);

impl TryFrom<CalendarQuery> for ParsedQuery {
    type Error = crate::Error;

    fn try_from(value: CalendarQuery) -> Result<Self, Self::Error> {
        let bare = value
            .bare
            .as_ref()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|u| serde_json::from_str(&u))
            .collect::<serde_json::Result<Vec<RepositoryInfo>>>()?;

        let gitea = value
            .gitea
            .as_ref()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|u| serde_json::from_str(&u))
            .collect::<serde_json::Result<Vec<RepositoryInfo>>>()?;

        let github = value.github.clone();
        let gitlab = value.gitlab.clone();
        let builder = value.into();

        Ok(Self(
            Repositories {
                github,
                gitlab,
                gitea,
                bare,
            },
            builder,
        ))
    }
}

impl CalendarQuery {}

impl From<CalendarQuery> for svg_renderer::Builder {
    fn from(query: CalendarQuery) -> Self {
        svg_renderer::Builder {
            cell_size: query.cell_size,
            colour_strategy: query.colour_strategy,
            font_size: query.font_size,
            active_colour: query.active_colour,
            inactive_colour: query.inactive_colour,
        }
    }
}
