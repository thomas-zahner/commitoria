use axum::{
    extract::Query,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use commitoria_lib::{
    provider::{github::Github, gitlab::Gitlab, GitProvider},
    source::ReqwestDataSource,
    svg::{
        contribution_colour::ColourStrategy, rgba::Rgba, SvgRenderer, SvgRendererBuilder,
        SvgRendererBuilderError,
    },
    types::{ContributionActivity, Error},
};
use serde::Deserialize;

macro_rules! static_file {
    ($file:expr, $content_type:expr $(,)?) => {{
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static($content_type));
        let content = include_str!(concat!("../static/", $file));
        let result: Result<(HeaderMap, &str), StatusCode> = Ok((headers, content));
        get(result)
    }};
}

#[derive(Deserialize, Clone)]
struct Names {
    github: Option<String>,
    gitlab: Option<String>,
    font_size: Option<usize>,
    cell_size: Option<usize>,
    colour_strategy: Option<ColourStrategy>,
}

async fn get_calendar_data(names: Query<Names>) -> Result<ContributionActivity, Error> {
    let mut activity = ContributionActivity::new();

    if let Some(name) = names.0.gitlab {
        activity += Gitlab::fetch(ReqwestDataSource {}, name).await?;
    }

    if let Some(name) = names.0.github {
        activity += Github::fetch(ReqwestDataSource {}, name).await?;
    }

    Ok(activity)
}

#[derive(Debug)]
enum BuilderError {
    SvgRendererBuilderError(SvgRendererBuilderError),
    UnknownStrategy(String),
}

impl From<SvgRendererBuilderError> for BuilderError {
    fn from(value: SvgRendererBuilderError) -> Self {
        Self::SvgRendererBuilderError(value)
    }
}

impl From<BuilderError> for (StatusCode, String) {
    fn from(error: BuilderError) -> Self {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error))
    }
}

fn build_renderer(names: Query<Names>) -> Result<SvgRenderer, BuilderError> {
    let mut builder = SvgRendererBuilder::default();

    if let Some(cell_size) = names.0.cell_size {
        builder.cell_size(cell_size);
    }

    if let Some(font_size) = names.0.font_size {
        builder.font_size(font_size);
    }

    if let Some(strategy) = names.0.colour_strategy {
        builder.colour_strategy(strategy);
    }

    Ok(builder.build()?)
}

async fn get_calendar_svg(names: Query<Names>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("image/svg+xml"));
    let activity = get_calendar_data(names.clone()).await?;
    Ok((headers, build_renderer(names)?.render(&activity)))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/calendar.svg", get(get_calendar_svg))
        .route_service("/", static_file!("gitlab-calendar/index.html", "text/html"))
        .route_service(
            "/calendar",
            static_file!("gitlab-calendar/calendar.html", "text/html"),
        )
        .route_service(
            "/calendar.js",
            static_file!("gitlab-calendar/calendar.js", "text/javascript"),
        )
        .route_service(
            "/main.css",
            static_file!("gitlab-calendar/main.css", "text/css"),
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
