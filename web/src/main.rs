use axum::{extract::Query, http::StatusCode, routing::get, Json, Router};
use commitoria_lib::{
    provider::{github::Github, gitlab::Gitlab, GitProvider},
    source::ReqwestDataSource,
    ContributionActivity,
};
use serde::Deserialize;
use tower_http::services::ServeFile;

#[derive(Deserialize)]
struct Names {
    github: Option<String>,
    gitlab: Option<String>,
}

async fn get_calendar_data(names: Query<Names>) -> Result<Json<ContributionActivity>, StatusCode> {
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

    Ok(Json(activity))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/calendar", get(get_calendar_data))
        .route_service("/", ServeFile::new("static/gitlab-calendar/index.html"))
        .route_service(
            "/calendar",
            ServeFile::new("static/gitlab-calendar/calendar.html"),
        )
        .route_service(
            "/calendar.js",
            ServeFile::new("static/gitlab-calendar/calendar.js"),
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
