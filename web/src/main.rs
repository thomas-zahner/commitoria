use axum::{http::StatusCode, routing::get, Json, Router};
use commitoria_core::{
    provider::{github::Github, GitProvider},
    source::ReqwestDataSource,
    ContributionActivity,
};

async fn get_calendar_data() -> Result<Json<ContributionActivity>, StatusCode> {
    match Github::fetch(ReqwestDataSource {}, "mre".into()).await {
        Ok(r) => Ok(Json(r)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_calendar_data));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
