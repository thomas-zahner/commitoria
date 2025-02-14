use axum::{
    extract::Query,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use commitoria_lib::{
    provider::{github::Github, gitlab::Gitlab, GitProvider},
    source::ReqwestDataSource,
    svg::SvgRenderer,
    types::ContributionActivity,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Names {
    github: Option<String>,
    gitlab: Option<String>,
}

async fn get_calendar_data(names: Query<Names>) -> Result<ContributionActivity, StatusCode> {
    let mut activity = ContributionActivity::new();

    if let Some(name) = names.0.gitlab {
        activity += Gitlab::fetch(ReqwestDataSource {}, name)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    if let Some(name) = names.0.github {
        activity += Github::fetch(ReqwestDataSource {}, name)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(activity)
}

async fn get_calendar_data_json(
    names: Query<Names>,
) -> Result<Json<ContributionActivity>, StatusCode> {
    get_calendar_data(names).await.map(|v| Json(v))
}

async fn get_calendar_svg(names: Query<Names>) -> Result<impl IntoResponse, StatusCode> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("image/svg+xml"));
    let activity = get_calendar_data(names);
    Ok((headers, SvgRenderer::render(&activity.await?)))
}

macro_rules! static_file {
    ($file:expr, $content_type:expr $(,)?) => {{
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static($content_type));
        let content = include_str!(concat!("../static/", $file));
        let result: Result<(HeaderMap, &str), StatusCode> = Ok((headers, content));
        get(result)
    }};
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/calendar", get(get_calendar_data_json))
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
