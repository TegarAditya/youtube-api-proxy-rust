use axum::{
    body::Body,
    http::{HeaderName, HeaderValue, Request, Response, header},
    middleware::Next,
};
use std::sync::OnceLock;

static HEADERS_TO_ADD: OnceLock<Vec<(HeaderName, HeaderValue)>> = OnceLock::new();

fn get_headers() -> &'static [(HeaderName, HeaderValue)] {
    HEADERS_TO_ADD.get_or_init(|| {
        vec![
            (
                header::STRICT_TRANSPORT_SECURITY,
                HeaderValue::from_static("max-age=63072000; includeSubDomains; preload"),
            ),
            (
                header::CONTENT_SECURITY_POLICY,
                HeaderValue::from_static("default-src 'self'"),
            ),
            (
                header::X_CONTENT_TYPE_OPTIONS,
                HeaderValue::from_static("nosniff"),
            ),
            (
                header::X_FRAME_OPTIONS, 
                HeaderValue::from_static("DENY")),
            (
                header::REFERRER_POLICY,
                HeaderValue::from_static("no-referrer"),
            ),
            (
                header::X_XSS_PROTECTION, 
                HeaderValue::from_static("0")
            ),
            (
                HeaderName::from_static("origin-agent-cluster"),
                HeaderValue::from_static("?1"),
            ),
            (
                HeaderName::from_static("x-dns-prefetch-control"),
                HeaderValue::from_static("off"),
            ),
            (
                HeaderName::from_static("x-download-options"),
                HeaderValue::from_static("noopen"),
            ),
            (
                HeaderName::from_static("x-permitted-cross-domain-policies"),
                HeaderValue::from_static("none"),
            ),
        ]
    })
}

pub async fn secure_headers(req: Request<Body>, next: Next) -> Response<Body> {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    headers.extend(get_headers().iter().cloned());

    headers.remove("x-powered-by");

    response
}
