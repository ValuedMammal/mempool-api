use crate::Transport;

pub extern crate reqwest;
pub extern crate tokio;

/// Base backoff in milliseconds.
const BASE_BACKOFF_MILLIS: u64 = 256;
/// Default max retries.
const DEFAULT_MAX_RETRIES: u32 = 10;

/// Wrapper for [`reqwest::Client`] to act as the transport mechanism.
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

impl Transport for ReqwestClient {
    type Resp = reqwest::Response;

    // TODO: Consider make new Error type
    type Err = reqwest::Error;

    async fn get<'a>(&'a self, path: &'a str) -> Result<Self::Resp, Self::Err>
    where
        Self: 'a,
    {
        self.send_retry(path).await
    }

    async fn post<'a>(&'a self, path: &'a str, body: String) -> Result<Self::Resp, Self::Err>
    where
        Self: 'a,
    {
        self.inner.post(path).body(body).send().await
    }

    async fn parse_response_text(&self, resp: Self::Resp) -> Result<String, Self::Err> {
        resp.text().await
    }

    async fn parse_response_raw(&self, resp: Self::Resp) -> Result<Vec<u8>, Self::Err> {
        Ok(resp.bytes().await?.to_vec())
    }

    async fn parse_response_json<'a, O>(&'a self, resp: Self::Resp) -> Result<O, Self::Err>
    where
        O: for<'de> serde::Deserialize<'de> + 'a,
    {
        resp.json().await
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
