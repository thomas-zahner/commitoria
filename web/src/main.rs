use crate::query::CalendarQuery;
use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_extra::extract::Query;
use commitoria_lib::{
    provider::{git::Repository, gitea::Gitea, github::Github, gitlab::Gitlab},
    source::ReqwestDataSource,
    svg::svg_renderer::SvgRenderer,
    types::{ContributionActivity, Error},
};
use const_format::concatcp;
use query::{ParsedQuery, Repositories};
use std::{net::SocketAddr, sync::Arc};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

mod query;

const MAX_SVG_CACHE_AGE_IN_SECONDS: usize = 60 * 60;
const RATE_LIMITING_INTERVAL_IN_SECONDS: u64 = 20;
const RATE_LIMITING_BURST_SIZE: u32 = 10;

macro_rules! static_file {
    ($file:expr, $content_type:expr $(,)?) => {{
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static($content_type));
        let content = include_str!(concat!("../static/", $file));
        let result: Result<(HeaderMap, &str), StatusCode> = Ok((headers, content));
        get(result)
    }};
}

fn get_svg_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("image/svg+xml"));
    headers.insert(
        "Cache-Control",
        HeaderValue::from_static(concatcp!("max-age=", MAX_SVG_CACHE_AGE_IN_SECONDS)),
    );
    headers
}

async fn get_calendar_data(repositories: Repositories) -> Result<ContributionActivity, Error> {
    let mut activity = ContributionActivity::new();

    if let Some(name) = &repositories.gitlab {
        activity += Gitlab::fetch(ReqwestDataSource {}, name.clone()).await?;
    }

    if let Some(name) = &repositories.github {
        activity += Github::fetch(ReqwestDataSource {}, name.clone()).await?;
    }

    for repository in repositories.bare {
        activity += Repository::new(repository.url)
            .await?
            .get_activity(repository.user_name)
            .await?;
    }

    for repository in repositories.gitea {
        activity +=
            Gitea::fetch(ReqwestDataSource {}, repository.user_name, repository.url).await?;
    }

    Ok(activity)
}

async fn get_calendar_svg(
    Query(query): Query<CalendarQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let ParsedQuery(repositories, builder) = query.try_into()?;
    let activity = get_calendar_data(repositories).await?;
    let result: Result<SvgRenderer, Error> = builder.build().map_err(|e| e.into());
    Ok((get_svg_headers(), result?.render(&activity)))
}

#[tokio::main]
async fn main() {
    let rate_limited_routes = Router::new()
        .route("/api/calendar.svg", get(get_calendar_svg))
        .layer(GovernorLayer {
            config: Arc::new(
                GovernorConfigBuilder::default()
                    .per_second(RATE_LIMITING_INTERVAL_IN_SECONDS)
                    .burst_size(RATE_LIMITING_BURST_SIZE)
                    .finish()
                    .unwrap(),
            ),
        });

    let static_routes = Router::new()
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

    let app = Router::new()
        .merge(rate_limited_routes)
        .merge(static_routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
