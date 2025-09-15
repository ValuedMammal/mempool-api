use core::fmt::{Debug, Display};
use core::future::Future;
use core::ops::Deref;

use serde::Deserialize;

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

    /// Parse a future response body as raw binary data.
    fn parse_response_raw(
        &self,
        resp: Self::Resp,
    ) -> impl Future<Output = Result<Vec<u8>, Self::Err>>;

    /// Parse a future response output that can be deserialized.
    fn parse_response_json<'a, O>(
        &'a self,
        resp: Self::Resp,
    ) -> impl Future<Output = Result<O, Self::Err>>
    where
        O: for<'de> Deserialize<'de> + 'a;
}

impl<T> Transport for T
where
    T: Deref,
    T::Target: Transport,
{
    type Resp = <T::Target as Transport>::Resp;

    type Err = <T::Target as Transport>::Err;

    fn get<'a>(&'a self, path: &'a str) -> impl Future<Output = Result<Self::Resp, Self::Err>>
    where
        Self: 'a,
    {
        (**self).get(path)
    }

    fn post<'a>(
        &'a self,
        path: &'a str,
        body: String,
    ) -> impl Future<Output = Result<Self::Resp, Self::Err>>
    where
        Self: 'a,
    {
        (**self).post(path, body)
    }

    fn parse_response_text(
        &self,
        resp: Self::Resp,
    ) -> impl Future<Output = Result<String, Self::Err>> {
        (**self).parse_response_text(resp)
    }

    fn parse_response_raw(
        &self,
        resp: Self::Resp,
    ) -> impl Future<Output = Result<Vec<u8>, Self::Err>> {
        (**self).parse_response_raw(resp)
    }

    fn parse_response_json<'a, O>(
        &'a self,
        resp: Self::Resp,
    ) -> impl Future<Output = Result<O, Self::Err>>
    where
        O: for<'de> Deserialize<'de> + 'a,
    {
        (**self).parse_response_json(resp)
    }
}
