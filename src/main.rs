#![forbid(unsafe_code)]
mod api_error;
mod asset_cache;
mod config;
mod routes;
mod state;

use std::{error, ffi::OsStr, time::Duration};

use axum::{
    extract::{Path, Request, State},
    http::{
        header::{ACCEPT, CONTENT_ENCODING, CONTENT_TYPE},
        HeaderMap, HeaderName, HeaderValue, Method, StatusCode,
    },
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use axum_extra::extract::cookie::Key;
use config::Config;
use deadpool::Runtime;
use deadpool_postgres::Config as PgConfig;
use minijinja::Environment;
use routes::{
    index::{about, index},
    not_found::not_found,
    robots::robots,
};
use state::SharedState;
use tokio_postgres::NoTls;
use tower_http::{
    compression::{predicate::SizeAbove, CompressionLayer},
    cors::CorsLayer,
    CompressionLevel,
};
use tracing_subscriber::{prelude::*, EnvFilter};

use crate::{asset_cache::AssetCache, routes::BaseTemplateData, state::AppState};

pub type BoxedError = Box<dyn error::Error>;

/// Leak a value as a static reference.
pub fn leak_alloc<T>(value: T) -> &'static T {
    Box::leak(Box::new(value))
}

#[tokio::main]
async fn main() -> Result<(), BoxedError> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let config = Config::new(".env");

    let pg = {
        let mut pg_config = PgConfig::new();
        pg_config.url = Some(config.postgres_url.clone());

        pg_config.create_pool(Some(Runtime::Tokio1), NoTls)?
    };

    let assets = leak_alloc(AssetCache::load_files().await);
    let base_template_data = leak_alloc(BaseTemplateData::new(assets));
    let env = import_templates()?;

    let app_state = leak_alloc(AppState {
        pg,
        assets,
        base_template_data,
        env,
        encryption_key: Key::from(config.encryption_key.as_bytes()),
    });

    let router = Router::new()
        .merge(route_handler(app_state))
        .nest("/api", api_handler(app_state))
        .nest("/assets", static_file_handler(app_state));

    tracing::info!("Listening on {}", config.addr());

    let listener = tokio::net::TcpListener::bind(&config.addr()).await?;

    axum::serve(
        listener,
        router
            .layer(
                CorsLayer::new()
                    .allow_credentials(true)
                    .allow_headers([ACCEPT, CONTENT_TYPE, HeaderName::from_static("csrf-token")])
                    .max_age(Duration::from_secs(86400))
                    .allow_origin(config.cors_origin)
                    .allow_methods([
                        Method::GET,
                        Method::POST,
                        Method::PUT,
                        Method::DELETE,
                        Method::OPTIONS,
                        Method::HEAD,
                        Method::PATCH,
                    ]),
            )
            .layer(
                CompressionLayer::new()
                    .quality(CompressionLevel::Precise(4))
                    .compress_when(SizeAbove::new(512)),
            )
            .into_make_service(),
    )
    .await?;

    Ok(())
}

fn static_file_handler(state: SharedState) -> Router {
    Router::new()
        .route(
            "/:file",
            get(|state: State<SharedState>, path: Path<String>| async move {
                let Some(asset) = state.assets.get_from_path(&path) else {
                    return StatusCode::NOT_FOUND.into_response();
                };

                let mut headers = HeaderMap::new();

                // We set the content type explicitly here as it will otherwise
                // be inferred as an `octet-stream`
                headers.insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static(asset.ext().unwrap_or("")),
                );

                if [Some("css"), Some("js")].contains(&asset.ext()) {
                    headers.insert(CONTENT_ENCODING, HeaderValue::from_static("br"));
                }

                // `bytes::Bytes` clones are cheap
                (headers, asset.contents.clone()).into_response()
            }),
        )
        .layer(middleware::from_fn(cache_control))
        .with_state(state)
}

fn route_handler(state: SharedState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/about", get(about))
        .route("/robots.txt", get(robots))
        .fallback(not_found)
        .with_state(state)
        .layer(middleware::from_fn(cache_control))
}

fn api_handler(state: SharedState) -> Router {
    Router::new()
        .route("/health", get(|| async { Html("OK") }))
        .fallback(not_found)
        .with_state(state)
}

fn import_templates() -> Result<Environment<'static>, BoxedError> {
    let mut env = Environment::new();

    for entry in std::fs::read_dir("templates")?.filter_map(Result::ok) {
        let path = entry.path();

        if path.is_file() && path.extension() == Some(OsStr::new("html")) {
            let name = path
                .file_name()
                .and_then(OsStr::to_str)
                .ok_or("failed to convert path to string")?
                .to_owned();

            let data = std::fs::read_to_string(&path)?;

            env.add_template_owned(name, data)?;
        }
    }

    Ok(env)
}

async fn cache_control(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;

    if let Some(content_type) = response.headers().get(CONTENT_TYPE) {
        const CACHEABLE_CONTENT_TYPES: [&str; 6] = [
            "text/css",
            "application/javascript",
            "image/svg+xml",
            "image/webp",
            "font/woff2",
            "image/png",
        ];

        if CACHEABLE_CONTENT_TYPES.iter().any(|&ct| content_type == ct) {
            let value = format!("public, max-age={}", 60 * 60 * 24);

            if let Ok(value) = HeaderValue::from_str(&value) {
                response.headers_mut().insert("cache-control", value);
            }
        }
    }

    response
}
