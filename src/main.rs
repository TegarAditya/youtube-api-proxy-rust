mod handler;
mod kv_store;
mod logger;
mod yt_client;

use crate::kv_store::KVStore;
use crate::logger::log_requests;
use crate::yt_client::YouTubeClient;
use axum::{
    Router,
    http::{HeaderValue, header},
    middleware::{self},
    routing::{delete, get},
};
use dotenvy::dotenv;
use std::env;
use tower::ServiceBuilder;
use tower_http::set_header::SetResponseHeaderLayer;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    kv_store: KVStore,
    yt_client: YouTubeClient,
    secret_key: String,
    cache_ttl_seconds: i64,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_ansi(false)
        .compact()
        .init();

    dotenv().ok();
    let youtube_api_key = env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY must be set");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let app_port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let cache_ttl_seconds = env::var("CACHE_TTL_SECONDS")
        .ok()
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(86400);

    let state = AppState {
        kv_store: KVStore::new("kv_store.sqlite").expect("Failed to initialize KVStore"),
        yt_client: YouTubeClient::new(youtube_api_key),
        secret_key,
        cache_ttl_seconds,
    };

    info!("✅ KV store and YouTube client initialized successfully");

    let middleware_stack = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::if_not_present(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=63072000; includeSubDomains; preload"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::CONTENT_SECURITY_POLICY,
            HeaderValue::from_static("default-src 'self'"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(middleware::from_fn(log_requests));

    let app = Router::new()
        .route("/api/video/{id}", get(handler::find_content))
        .route("/api/video/clear", delete(handler::clear_cache))
        .route("/healthz", get(handler::health_check))
        .with_state(state)
        .layer(middleware_stack);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", app_port.parse::<u16>().unwrap()))
        .await
        .unwrap();

    info!("✅ Server started successfully on http://0.0.0.0:{app_port}");

    axum::serve(listener, app).await.unwrap();
}
