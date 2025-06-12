mod kv_store;

use crate::kv_store::{KVStore, KeyValue};
use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
};
use serde::{Deserialize};

#[tokio::main]
async fn main() {
    let store = KVStore::new("kv_store.sqlite").expect("Failed to initialize KVStore");

    println!("✅ KV store initialized successfully");

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/kv/:key", get(get_key).post(set_key).delete(delete_key))
        .with_state(store);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("✅ Server started successfully on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> &'static str {
    "Hello, World from Axum with SQLite!"
}

#[derive(Deserialize)]
struct SetValuePayload {
    value: String,
}

async fn set_key(
    State(store): State<KVStore>,
    Path(key): Path<String>,
    Json(payload): Json<SetValuePayload>,
) -> StatusCode {
    match store.set(&key, &payload.value) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn get_key(
    State(store): State<KVStore>,
    Path(key): Path<String>,
) -> Result<Json<KeyValue>, StatusCode> {
    match store.get(&key) {
        Ok(Some(kv)) => Ok(Json(kv)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_key(State(store): State<KVStore>, Path(key): Path<String>) -> StatusCode {
    match store.delete(&key) {
        Ok(0) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
