use crate::Transport;

/// Wrapper for [`reqwest::Client`] to act as the transport mechanism.
#[derive(Debug)]
pub struct ReqwestClient {
    /// inner `reqwest` client.
    pub inner: reqwest::Client,
}

impl Default for ReqwestClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ReqwestClient {
    /// New.
    pub fn new() -> Self {
        Self {
            inner: reqwest::Client::new(),
        }
    }
}

impl Transport for ReqwestClient {
    type Resp = reqwest::Response;

    type Err = reqwest::Error;

    async fn get<'a>(&'a self, path: &'a str) -> Result<Self::Resp, Self::Err>
    where
        Self: 'a,
    {
        self.inner.get(path).send().await
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
