use axum::{
    body::Body,
    http::{Request, Response},
    middleware::Next,
};
use std::time::Instant;
use tracing::info;

pub async fn log_requests(req: Request<Body>, next: Next) -> Response<Body> {
    let method = req.method().clone();
    let uri = req.uri().clone();

    info!("<-- {} {}", method, uri.path());

    let start = Instant::now();
    let response = next.run(req).await;
    let latency = start.elapsed();

    info!(
        "--> {} {} {} {}ms",
        method,
        uri.path(),
        response.status().as_u16(),
        latency.as_millis()
    );

    response
}
