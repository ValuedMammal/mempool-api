use core::fmt::{Debug, Display};
use core::future::Future;
use core::ops::Deref;

/// Trait describing the behavior required of the HTTP client.
pub trait Http {
    /// Body
    type Body: AsRef<[u8]>;

    /// Error
    type Err: Debug + Display;

    /// Sends a GET request to the given `path`.
    fn get<'a>(&'a self, path: &'a str) -> impl Future<Output = Result<Self::Body, Self::Err>>
    where
        Self: 'a;

    /// Sends a POST request to `path` with text body.
    fn post<'a>(
        &'a self,
        path: &'a str,
        body: String,
    ) -> impl Future<Output = Result<Self::Body, Self::Err>>
    where
        Self: 'a;
}

impl<T> Http for T
where
    T: Deref,
    T::Target: Http,
{
    type Body = <T::Target as Http>::Body;

    type Err = <T::Target as Http>::Err;

    fn get<'a>(&'a self, path: &'a str) -> impl Future<Output = Result<Self::Body, Self::Err>>
    where
        Self: 'a,
    {
        (**self).get(path)
    }

    fn post<'a>(
        &'a self,
        path: &'a str,
        body: String,
    ) -> impl Future<Output = Result<Self::Body, Self::Err>>
    where
        Self: 'a,
    {
        (**self).post(path, body)
    }
}
