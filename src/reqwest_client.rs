use crate::Transport;

/// Wrapper for [`reqwest::Client`] to act as the transport mechanism.
#[derive(Debug)]
pub struct ReqwestClient {
    /// inner `reqwest` client.
    pub inner: reqwest::Client,
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

    fn get<'a>(&'a self, path: &'a str) -> impl Future<Output = Result<Self::Resp, Self::Err>>
    where
        Self: 'a,
    {
        async move { self.inner.get(path).send().await }
    }

    fn post<'a>(
        &'a self,
        path: &'a str,
        body: String,
    ) -> impl Future<Output = Result<Self::Resp, Self::Err>>
    where
        Self: 'a,
    {
        async move { Ok(self.inner.post(path).body(body).send().await?) }
    }

    fn parse_response_text(
        &self,
        resp: Self::Resp,
    ) -> impl Future<Output = Result<String, Self::Err>> {
        async move { resp.text().await }
    }

    fn parse_response_json<'a, O>(
        &'a self,
        resp: Self::Resp,
    ) -> impl Future<Output = Result<O, Self::Err>>
    where
        O: for<'de> serde::Deserialize<'de> + 'a,
    {
        async move { resp.json().await }
    }
}
