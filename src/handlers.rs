use crate::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use chrono::{Duration, Utc};
use serde::Deserialize;
use serde_json;

// --- Utility Function ---

/// Checks if the cached data is still valid based on the TTL (Time To Live).
fn is_cache_valid(cached_at: &chrono::DateTime<Utc>, ttl_seconds: i64) -> bool {
    let expiration_time = *cached_at + Duration::seconds(ttl_seconds);
    expiration_time > Utc::now()
}

// --- Handlers for the API Endpoints ---

/// Handler to find content by YouTube video ID.
pub async fn find_content(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if let Ok(Some(cached)) = state.kv_store.get(&id) {
        if is_cache_valid(&cached.cached_at, state.cache_ttl_seconds) {
            println!("--- HIT {id}; returning cached data.");

            let json_value: serde_json::Value =
                serde_json::from_str(&cached.value).unwrap_or(serde_json::Value::Null);
            return Ok(Json(json_value));
        }
    }

    println!("--- MISS {id}; fetching new data.");

    if !state.yt_client.is_valid_video_id(&id).await {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid YouTube video ID".to_string(),
        ));
    }

    match state.yt_client.get_video_data(&id).await {
        Ok(new_data) => {
            if let Ok(json_string) = serde_json::to_string(&new_data) {
                let _ = state.kv_store.set(&id, &json_string);
            }

            match serde_json::to_value(new_data) {
                Ok(json_value) => Ok(Json(json_value)),
                Err(_) => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to serialize response".to_string(),
                )),
            }
        }
        Err(e) => Err((StatusCode::NOT_FOUND, format!("No data found: {}", e))),
    }
}

// --- Protected Handlers ---

#[derive(Deserialize)]
pub struct ClearCacheQuery {
    key: String,
}

/// Handler to clear the cache, protected by a secret key.
pub async fn clear_cache(
    State(state): State<AppState>,
    Query(query): Query<ClearCacheQuery>,
) -> impl IntoResponse {
    if query.key != state.secret_key {
        return (StatusCode::UNAUTHORIZED, "Unauthorized".to_string());
    }

    match state.kv_store.clear() {
        Ok(_) => (StatusCode::OK, "Cache cleared successfully".to_string()),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error clearing cache: {}", e),
        ),
    }
}

// --- Health Check Handler ---

/// Handler for the health check endpoint.
pub async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    match state.kv_store.health_check() {
        Ok(_) => (StatusCode::OK, "OK"),
        Err(_) => (StatusCode::SERVICE_UNAVAILABLE, "Service Unavailable"),
    }
}