use core::fmt;

use bitreq::{Request, Response};
use bytes::Bytes;

use crate::{Http, HttpMethod};

pub extern crate bitreq;
pub extern crate tokio;

/// Base backoff in milliseconds.
const BASE_BACKOFF_MILLIS: u64 = 256;
/// Default max retries.
const DEFAULT_MAX_RETRIES: u32 = 6;

/// HTTP client implementation.
#[derive(Debug, Clone)]
pub struct BitreqClient {
    /// The maximum number of times to retry a failed request.
    max_retries: u32,
}

impl BitreqClient {
    /// Creates a new default `bitreq` client.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `bitreq` client builder.
    pub fn builder() -> BitreqClientBuilder {
        BitreqClientBuilder {
            inner: BitreqClient::new(),
        }
    }
}

impl Default for BitreqClient {
    fn default() -> Self {
        Self {
            max_retries: DEFAULT_MAX_RETRIES,
        }
    }
}

/// Builder struct for [`BitreqClient`].
#[derive(Debug)]
pub struct BitreqClientBuilder {
    /// The inner client.
    inner: BitreqClient,
}

impl BitreqClientBuilder {
    /// Set the maximum number of times to retry a request. Note not all failed requests
    /// are able to be retried.
    pub fn retries(mut self, max_retries: u32) -> Self {
        self.inner.max_retries = max_retries;
        self
    }

    /// Returns the `bitreq` client.
    pub fn build(self) -> BitreqClient {
        self.inner
    }
}

impl Http for BitreqClient {
    type Body = Bytes;

    type Err = BitreqError;

    async fn send<'a>(
        &'a self,
        method: HttpMethod,
        url: &'a str,
        body: impl Into<Self::Body>,
    ) -> Result<Self::Body, Self::Err>
    where
        Self: 'a,
    {
        let resp = self.send_retry(method.into(), url, body.into()).await?;

        if !is_status_ok(resp.status_code) {
            return Err(BitreqError::HttpResponse {
                status: resp.status_code,
                message: resp.reason_phrase,
            });
        }

        Ok(resp.into_bytes().into())
    }
}

impl BitreqClient {
    /// Sends a request and allows for retrying failed attempts. See [`is_status_retryable`].
    async fn send_retry(
        &self,
        method: bitreq::Method,
        url: &str,
        body: Bytes,
    ) -> Result<Response, bitreq::Error> {
        let mut delay = BASE_BACKOFF_MILLIS;
        let mut attempts = 0;

        loop {
            match Request::new(method.clone(), url)
                .with_body(body.clone())
                .send_async()
                .await?
            {
                resp if attempts < self.max_retries && is_status_retryable(resp.status_code) => {
                    tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                    delay *= 2;
                    attempts += 1;
                }
                resp => return Ok(resp),
            }
        }
    }
}

/// Whether the response status indicates a failure which can be retried.
///
/// Currently includes:
///
/// - `429`: TOO_MANY_REQUESTS
/// - `500`: INTERNAL_SERVER_ERROR
/// - `503`: SERVICE_UNAVAILABLE
fn is_status_retryable(status: i32) -> bool {
    [429, 500, 503].contains(&status)
}

/// Whether the response status code is `200 OK`.
fn is_status_ok(status: i32) -> bool {
    status == 200
}

/// Error for `BitreqClient`
#[derive(Debug)]
pub enum BitreqError {
    /// `bitreq` error.
    Bitreq(bitreq::Error),
    /// Reponse error.
    HttpResponse { status: i32, message: String },
}

impl fmt::Display for BitreqError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bitreq(e) => write!(f, "{e}"),
            Self::HttpResponse { status, message } => write!(f, "{status} {message}"),
        }
    }
}

impl std::error::Error for BitreqError {}

impl From<bitreq::Error> for BitreqError {
    fn from(e: bitreq::Error) -> Self {
        Self::Bitreq(e)
    }
}
