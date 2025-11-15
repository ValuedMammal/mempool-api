# `mempool_space_api`

## Getting started

<!-- This is here for illustration. The example doesn't compile because we don't use reqwest! -->
```rust,compile_fail
use bytes::Bytes;
use mempool_space_api::AsyncClient;
use mempool_space_api::{Http, HttpMethod};

// Define a HTTP client implementation.

#[derive(Debug)]
struct MyClient {
    inner: reqwest::Client,
}

// Implement `Http` for `MyClient`.

impl Http for MyClient {
    type Body = Bytes;
    type Err = reqwest::Error;

    async fn send<'a>(
        &'a self,
        method: HttpMethod,
        url: &'a str,
        body: impl Into<Self::Body>,
    ) -> Result<Self::Body, Self::Err>
    where
        Self: 'a,
    {
        let resp = match method {
            HttpMethod::GET => self.inner.get(url).send().await?,
            HttpMethod::POST => self.inner.post(url).body(body.into()).send().await?,
        };

        resp.bytes().await
    }
}

// Start making API requests.

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let my_client = MyClient {
        inner: reqwest::Client::new(),
    };
    let client = AsyncClient::new("https://mempool.space/api", &my_client);

    // Get recommended fees.
    let res = client.get_recommended_fees().await?;
    println!("{res:#?}");

    Ok(())
}
```

## Features

* `bitreq`: An async HTTP client that can be used with this library out of the box.

