use std::{
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use axum::{
    http::{
        header::{CACHE_CONTROL, CONTENT_TYPE},
        HeaderValue, Request,
    },
    response::Response,
};
use futures_core::ready;
use pin_project_lite::pin_project;
use tower::{Layer, Service};

use crate::mime::MimeType;

pub const CACHEABLE_MIME_TYPES: [MimeType; 5] = [
    MimeType::CSS,
    MimeType::JS,
    MimeType::SVG,
    MimeType::WEBP,
    MimeType::WOFF2,
];

#[derive(Debug, Clone)]
pub struct CacheControlLayer<'a> {
    cacheable_mime_types: &'a [MimeType],
    max_age: HeaderValue,
}

impl<'a> Default for CacheControlLayer<'a> {
    fn default() -> Self {
        Self {
            cacheable_mime_types: &CACHEABLE_MIME_TYPES,
            max_age: HeaderValue::from_static("max-age=31536000"),
        }
    }
}

impl<'a> CacheControlLayer<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_cacheable_mime_types(mut self, mime_types: &'a [MimeType]) -> Self {
        self.cacheable_mime_types = mime_types;
        self
    }

    pub fn with_max_age(mut self, max_age: HeaderValue) -> Self {
        self.max_age = max_age;
        self
    }
}

impl<'a, S> Layer<S> for CacheControlLayer<'a> {
    type Service = CacheControl<'a, S>;

    fn layer(&self, inner: S) -> Self::Service {
        CacheControl {
            inner,
            layer: self.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CacheControl<'a, S> {
    inner: S,
    layer: CacheControlLayer<'a>,
}

impl<'a, S, T, U> Service<Request<T>> for CacheControl<'a, S>
where
    S: Service<Request<T>, Response = Response<U>>,
    U: Default,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<'a, S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<T>) -> Self::Future {
        let response_future = self.inner.call(req);

        ResponseFuture {
            response_future,
            layer: self.layer.clone(),
        }
    }
}

pin_project! {
    pub struct ResponseFuture<'a, F> {
        #[pin]
        response_future: F,
        layer: CacheControlLayer<'a>,
    }
}

impl<'a, F, B, E> Future for ResponseFuture<'a, F>
where
    F: Future<Output = Result<Response<B>, E>>,
    B: Default,
{
    type Output = Result<Response<B>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let mut response: Response<B> = ready!(this.response_future.poll(cx))?;

        if let Some(content_type) = response.headers().get(CONTENT_TYPE) {
            let mime = MimeType::from(content_type);

            if mime == MimeType::HTML {
                response
                    .headers_mut()
                    .insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));

                return Poll::Ready(Ok(response));
            }

            if this.layer.cacheable_mime_types.contains(&mime) {
                response
                    .headers_mut()
                    .insert(CACHE_CONTROL, this.layer.max_age.clone());
            }
        }

        Poll::Ready(Ok(response))
    }
}

#[derive(Debug, Default)]
struct CacheControlError;

impl fmt::Display for CacheControlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CacheControlError")
    }
}

impl std::error::Error for CacheControlError {}
