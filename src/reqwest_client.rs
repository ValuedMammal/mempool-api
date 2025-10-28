use core::fmt;

use bytes::Bytes;

use crate::{Http, HttpMethod};
pub extern crate reqwest;
pub extern crate tokio;

/// Base backoff in milliseconds.
const BASE_BACKOFF_MILLIS: u64 = 256;
/// Default max retries.
const DEFAULT_MAX_RETRIES: u32 = 10;

/// Wrapper for [`reqwest::Client`] to act as the HTTP implementation.
#[derive(Debug)]
pub struct ReqwestClient {
    /// inner `reqwest` client.
    pub inner: reqwest::Client,
    /// The maximum number of times to retry a failed request.
    max_retries: u32,
}

/// Reqwest client config builder.
#[derive(Debug)]
pub struct Config {
    client: ReqwestClient,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            client: ReqwestClient {
                inner: reqwest::Client::default(),
                max_retries: DEFAULT_MAX_RETRIES,
            },
        }
    }
}

impl Config {
    /// Set the maximum number of times to retry a failed request.
    pub fn max_retries(mut self, n: u32) -> Self {
        self.client.max_retries = n;
        self
    }

    /// Build.
    pub fn build(self) -> ReqwestClient {
        self.client
    }
}

impl Default for ReqwestClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ReqwestClient {
    /// New with default config.
    pub fn new() -> Self {
        Config::default().build()
    }

    /// Return a new reqwest client [`Config`].
    pub fn config() -> Config {
        Config::default()
    }
}

impl Http for ReqwestClient {
    type Body = Bytes;

    type Err = ReqwestError;

    async fn send<'a>(
        &'a self,
        method: HttpMethod,
        url: &'a str,
        body: impl Into<Self::Body>,
    ) -> Result<Self::Body, Self::Err>
    where
        Self: 'a,
    {
        let resp = self.send_retry(method, url, body.into()).await?;

        if !resp.status().is_success() {
            return Err(ReqwestError::HttpResponse {
                status: resp.status().as_u16(),
                message: resp.text().await?,
            });
        }

        Ok(resp.bytes().await?)
    }
}

impl ReqwestClient {
    /// Sends a request and allows for retrying failed attempts. See [`is_status_retryable`].
    async fn send_retry(
        &self,
        method: HttpMethod,
        url: &str,
        body: Bytes,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let mut delay = BASE_BACKOFF_MILLIS;
        let mut attempts = 0;

        loop {
            let request = match method {
                HttpMethod::GET => self.inner.get(url),
                HttpMethod::POST => self.inner.post(url).body(body.clone()),
            };
            match request.send().await? {
                resp if attempts < self.max_retries && is_status_retryable(resp.status()) => {
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
fn is_status_retryable(status: reqwest::StatusCode) -> bool {
    [429, 500, 503].contains(&status.as_u16())
}

/// Error for `ReqwestClient`
#[derive(Debug)]
pub enum ReqwestError {
    /// `reqwest` error.
    Reqwest(reqwest::Error),
    /// Reponse error.
    HttpResponse { status: u16, message: String },
}

impl fmt::Display for ReqwestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reqwest(e) => write!(f, "{e}"),
            Self::HttpResponse { status, message } => write!(f, "{status} {message}"),
        }
    }
}

impl std::error::Error for ReqwestError {}

impl From<reqwest::Error> for ReqwestError {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}
