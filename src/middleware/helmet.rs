use axum::{
    body::Body,
    http::{HeaderName, Request, Response, header},
    middleware::Next,
};

pub async fn secure_headers(req: Request<Body>, next: Next) -> Response<Body> {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    headers.insert(
        header::STRICT_TRANSPORT_SECURITY,
        "max-age=63072000; includeSubDomains; preload"
            .parse()
            .unwrap(),
    );

    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        "default-src 'self'"
            .parse()
            .unwrap(),
    );

    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS, 
        "nosniff"
            .parse()
            .unwrap()
    );

    headers.insert(
        header::X_FRAME_OPTIONS, 
        "DENY"
            .parse()
            .unwrap()
    );

    headers.insert(
        header::REFERRER_POLICY, 
        "no-referrer"
            .parse()
            .unwrap()
    );

    headers.insert(
        HeaderName::from_static("origin-agent-cluster"),
        "?1"
            .parse()
            .unwrap(),
    );

    headers.insert(
        HeaderName::from_static("x-dns-prefetch-control"),
        "off"
            .parse()
            .unwrap(),
    );

    headers.insert(
        HeaderName::from_static("x-download-options"),
        "noopen"
            .parse()
            .unwrap(),
    );

    headers.insert(
        HeaderName::from_static("x-permitted-cross-domain-policies"),
        "none"
            .parse()
            .unwrap(),
    );

    headers.insert(
        HeaderName::from_static("x-xss-protection"),
        "0"
            .parse()
            .unwrap(),
    );

    headers.remove("x-powered-by");

    response
}
