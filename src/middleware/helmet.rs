use axum::{
    body::Body,
    http::{HeaderName, HeaderValue, Request, Response, header},
};
use std::{
    future::Future,
    pin::Pin,
    sync::OnceLock,
    task::{Context, Poll},
};
use tower::{Layer, Service};

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
            (header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY")),
            (
                header::REFERRER_POLICY,
                HeaderValue::from_static("no-referrer"),
            ),
            (header::X_XSS_PROTECTION, HeaderValue::from_static("0")),
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

#[derive(Clone)]
pub struct SecureHeadersLayer;

impl<S> Layer<S> for SecureHeadersLayer {
    type Service = SecureHeadersMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SecureHeadersMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct SecureHeadersMiddleware<S> {
    inner: S,
}

impl<S, ReqBody> Service<Request<ReqBody>> for SecureHeadersMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
{
    type Response = Response<Body>;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let mut inner = self.inner.clone();
        let fut = inner.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            let headers = res.headers_mut();
            headers.extend(get_headers().iter().cloned());
            headers.remove("x-powered-by");

            Ok(res)
        })
    }
}
