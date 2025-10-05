use core::fmt;

use bytes::Bytes;

use crate::Http;
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

/// Reqwest client config.
#[derive(Debug)]
#[non_exhaustive]
pub struct Config {
    /// The maximum number of times to retry a failed request.
    pub max_retries: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_retries: DEFAULT_MAX_RETRIES,
        }
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
        Self::with_conf(Config::default())
    }

    /// Return a new reqwest client [`Config`].
    pub fn config() -> Config {
        Config::default()
    }

    /// New with the provided [`Config`].
    pub fn with_conf(conf: Config) -> Self {
        Self {
            inner: reqwest::Client::new(),
            max_retries: conf.max_retries,
        }
    }
}

impl Http for ReqwestClient {
    type Body = Bytes;

    type Err = ReqwestError;

    async fn get<'a>(&'a self, path: &'a str) -> Result<Self::Body, Self::Err>
    where
        Self: 'a,
    {
        let resp = self.send_retry(path).await?;
        if !resp.status().is_success() {
            return Err(ReqwestError::HttpResponse {
                status: resp.status().as_u16(),
                message: resp.text().await?,
            });
        }
        Ok(resp.bytes().await?)
    }

    async fn post<'a>(&'a self, path: &'a str, body: String) -> Result<Self::Body, Self::Err>
    where
        Self: 'a,
    {
        let resp = self.inner.post(path).body(body).send().await?;
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
    /// Send and retry.
    async fn send_retry(&self, path: &str) -> Result<reqwest::Response, reqwest::Error> {
        let mut delay = BASE_BACKOFF_MILLIS;
        let mut attempts = 0;

        loop {
            match self.inner.get(path).send().await? {
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
