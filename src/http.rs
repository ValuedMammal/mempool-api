use core::fmt::{Debug, Display};
use core::future::Future;
use core::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Method {
    Get,
    Post,
}

/// HTTP method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HttpMethod(Method);

impl HttpMethod {
    /// GET.
    pub const GET: Self = Self(Method::Get);
    /// POST.
    pub const POST: Self = Self(Method::Post);
}

/// Trait describing the behavior required of the HTTP client.
pub trait Http {
    /// Body
    type Body: AsRef<[u8]> + From<Vec<u8>>;

    /// Error
    type Err: Debug + Display;

    /// Send a request to a `url` and return a future response body.
    fn send<'a>(
        &'a self,
        method: HttpMethod,
        url: &'a str,
        body: impl Into<Self::Body>,
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

    fn send<'a>(
        &'a self,
        method: HttpMethod,
        url: &'a str,
        body: impl Into<Self::Body>,
    ) -> impl Future<Output = Result<Self::Body, Self::Err>>
    where
        Self: 'a,
    {
        (**self).send(method, url, body)
    }
}
