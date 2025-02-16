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
    svg::{SvgRenderer, SvgRendererBuilder},
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
}

async fn get_calendar_data(names: Query<Names>) -> Result<ContributionActivity, Error> {
    let mut activity = ContributionActivity::new();

    if let Some(name) = names.0.gitlab.clone() {
        activity += Gitlab::fetch(ReqwestDataSource {}, name).await?;
    }

    if let Some(name) = names.0.github.clone() {
        activity += Github::fetch(ReqwestDataSource {}, name).await?;
    }

    Ok(activity)
}

fn build_renderer(names: Query<Names>) -> SvgRenderer {
    let mut builder = SvgRendererBuilder::default();

    if let Some(cell_size) = names.0.cell_size {
        builder.cell_size(cell_size);
    }

    if let Some(font_size) = names.0.font_size {
        builder.font_size(font_size);
    }

    builder.build().unwrap()
}

async fn get_calendar_svg(names: Query<Names>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("image/svg+xml"));
    let activity = get_calendar_data(names.clone()).await?;
    Ok((headers, build_renderer(names).render(&activity)))
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
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
