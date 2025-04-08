use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_extra::extract::Query;
use commitoria_lib::{
    provider::{github::Github, gitlab::Gitlab, GitProvider},
    source::ReqwestDataSource,
    svg::svg_renderer::{Builder, SvgRenderer},
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
struct CalendarQuery {
    github: Option<String>,
    gitlab: Option<String>,
    font_size: Option<usize>,
    cell_size: Option<usize>,
    colour_strategy: Option<String>,
    active_colour: Option<String>,
    inactive_colour: Option<String>,
    git_urls: Option<Vec<String>>,
}

async fn get_calendar_data(query: Query<CalendarQuery>) -> Result<ContributionActivity, Error> {
    let mut activity = ContributionActivity::new();

    if let Some(name) = query.0.gitlab {
        activity += Gitlab::fetch(ReqwestDataSource {}, name).await?;
    }

    if let Some(name) = query.0.github {
        activity += Github::fetch(ReqwestDataSource {}, name).await?;
    }

    todo!("Fetch raw git repositories {:?}", query.0.git_urls);

    Ok(activity)
}

impl From<CalendarQuery> for Builder {
    fn from(query: CalendarQuery) -> Self {
        Builder {
            cell_size: query.cell_size,
            colour_strategy: query.colour_strategy.clone(),
            font_size: query.font_size,
            active_colour: query.active_colour.clone(),
            inactive_colour: query.inactive_colour.clone(),
        }
    }
}

async fn get_calendar_svg(
    query: Query<CalendarQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("image/svg+xml"));
    let activity = get_calendar_data(query.clone()).await?;
    let builder: Builder = query.0.into();
    let result: Result<SvgRenderer, Error> = builder.build().map_err(|e| e.into());
    Ok((headers, result?.render(&activity)))
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
            "/form.js",
            static_file!("gitlab-calendar/form.js", "text/javascript"),
        )
        .route_service(
            "/main.css",
            static_file!("gitlab-calendar/main.css", "text/css"),
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
