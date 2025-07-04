use axum::{
    body::Body,
    http::{Request, Response},
};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};
use tower::{Layer, Service};
use tracing::info;

#[derive(Clone)]
pub struct LoggerLayer;

impl<S> Layer<S> for LoggerLayer {
    type Service = LoggerMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LoggerMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct LoggerMiddleware<S> {
    inner: S,
}

impl<S, ReqBody> Service<Request<ReqBody>> for LoggerMiddleware<S>
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
        let method = req.method().clone();
        let uri = req.uri().clone();

        let mut inner = self.inner.clone();

        Box::pin(async move {
            if uri.path() == "/healthz" {
                return inner.call(req).await;
            }

            info!("<-- {} {}", method, uri.path());
            let start = Instant::now();

            let response = inner.call(req).await?;

            let elapsed = start.elapsed();
            info!(
                "--> {} {} {} {}ms",
                method,
                uri.path(),
                response.status().as_u16(),
                elapsed.as_millis()
            );

            Ok(response)
        })
    }
}
