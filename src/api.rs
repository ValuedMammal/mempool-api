//! [`api`](self).

use core::fmt::{Debug, Display};
use core::future::Future;

use serde::Deserialize;

/// Recommended fees.
#[derive(Debug, Deserialize)]
pub struct RecommendedFees {
    /// Fastest fee.
    #[serde(alias = "fastestFee")]
    pub fastest_fee: u64,
    /// Half-hour fee.
    #[serde(alias = "halfHourFee")]
    pub half_hour_fee: u64,
    /// Hour fee.
    #[serde(alias = "hourFee")]
    pub hour_fee: u64,
    /// Economy fee.
    #[serde(alias = "economyFee")]
    pub economy_fee: u64,
    /// Minimum fee.
    #[serde(alias = "minimumFee")]
    pub minimum_fee: u64,
}

/// Trait describing the behavior required of the I/O transport mechanism.
pub trait Transport {
    /// Response
    type Resp;

    /// Error
    type Err: Debug + Display;

    /// Sends a GET request to the given `path`.
    fn get<'a>(&'a self, path: &'a str) -> impl Future<Output = Result<Self::Resp, Self::Err>>
    where
        Self: 'a;

    /// Sends a POST request to `path` with text body.
    fn post<'a>(
        &'a self,
        path: &'a str,
        body: String,
    ) -> impl Future<Output = Result<Self::Resp, Self::Err>>
    where
        Self: 'a;

    /// Parse a future response body as a UTF-8 string.
    fn parse_response_text(
        &self,
        resp: Self::Resp,
    ) -> impl Future<Output = Result<String, Self::Err>>;

    /// Parse a future response output that can be deserialized.
    fn parse_response_json<'a, O>(
        &'a self,
        resp: Self::Resp,
    ) -> impl Future<Output = Result<O, Self::Err>>
    where
        O: for<'de> Deserialize<'de> + 'a;
}
