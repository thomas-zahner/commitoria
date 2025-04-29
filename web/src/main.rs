use std::{net::SocketAddr, sync::Arc};

use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_extra::extract::Query;
use commitoria_lib::{
    provider::{
        git::{Repository, RepositoryInfo},
        github::Github,
        gitlab::Gitlab,
    },
    source::ReqwestDataSource,
    svg::svg_renderer::{Builder, SvgRenderer},
    types::{ContributionActivity, Error},
};
use const_format::concatcp;
use serde::Deserialize;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

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

#[derive(Deserialize, Clone)]
struct CalendarQuery {
    github: Option<String>,
    gitlab: Option<String>,
    font_size: Option<usize>,
    cell_size: Option<usize>,
    colour_strategy: Option<String>,
    active_colour: Option<String>,
    inactive_colour: Option<String>,
    bare_repository: Option<Vec<String>>,
}

impl CalendarQuery {
    fn bare_repositories(&self) -> serde_json::Result<Vec<RepositoryInfo>> {
        self.bare_repository
            .as_ref()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|u| serde_json::from_str(&u))
            .collect()
    }
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

async fn get_calendar_data(query: CalendarQuery) -> Result<ContributionActivity, Error> {
    let mut activity = ContributionActivity::new();

    if let Some(name) = &query.gitlab {
        activity += Gitlab::fetch(ReqwestDataSource {}, name.clone()).await?;
    }

    if let Some(name) = &query.github {
        activity += Github::fetch(ReqwestDataSource {}, name.clone()).await?;
    }

    for repository in query.bare_repositories()? {
        activity += Repository::new(repository.url)
            .await?
            .get_activity(repository.user_name)
            .await?;
    }

    Ok(activity)
}

impl From<CalendarQuery> for Builder {
    fn from(query: CalendarQuery) -> Self {
        Builder {
            cell_size: query.cell_size,
            colour_strategy: query.colour_strategy,
            font_size: query.font_size,
            active_colour: query.active_colour,
            inactive_colour: query.inactive_colour,
        }
    }
}

async fn get_calendar_svg(
    Query(query): Query<CalendarQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let activity = get_calendar_data(query.clone()).await?;
    let builder: Builder = query.into();
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
