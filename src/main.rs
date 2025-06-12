use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("✅ Server started successfully on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> &'static str {
    "Hello, World from Axum!"
}
