use crate::headers::Headers;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use serde::de::DeserializeOwned;
use std::{fmt, pin::Pin};

// TODO: Should be pub(crate) in actual implementation.
pub type PinnedStream = Pin<Box<dyn Stream<Item = crate::Result<Bytes>> + Send + Sync>>;

pub struct Response {
    status: u16,
    headers: Headers,
    body: ResponseBody,
}

impl Response {
    pub fn new(status: u16, headers: Headers, stream: PinnedStream) -> Self {
        Self {
            status,
            headers,
            body: ResponseBody::new(stream),
        }
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn deconstruct(self) -> (u16, Headers, ResponseBody) {
        (self.status, self.headers, self.body)
    }

    pub fn into_body(self) -> ResponseBody {
        self.body
    }

    pub async fn json<T>(self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        self.into_body().json().await
    }
}

impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("status", &self.status)
            .field("headers", &self.headers)
            .field("body", &"(body)")
            .finish()
    }
}

#[pin_project::pin_project]
pub struct ResponseBody(#[pin] PinnedStream);

impl ResponseBody {
    pub fn new(stream: PinnedStream) -> Self {
        Self(stream)
    }

    pub async fn collect(mut self) -> crate::Result<Bytes> {
        let mut result = Vec::new();
        while let Some(res) = self.0.next().await {
            result.extend(&res?);
        }

        Ok(result.into())
    }

    pub async fn json<T>(self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        let body = self.collect().await?;
        crate::json::from_json(body)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct CollectedResponse {
    status: u16,
    headers: Headers,
    body: Bytes,
}

impl AsRef<[u8]> for CollectedResponse {
    fn as_ref(&self) -> &[u8] {
        self.body.as_ref()
    }
}

impl fmt::Debug for CollectedResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CollectedResponse")
            .field("status", &self.status)
            .field("headers", &self.headers)
            .field(
                "body",
                &String::from_utf8(self.body.to_vec()).unwrap_or_else(|_| String::from("(binary)")),
            )
            .finish()
    }
}

impl CollectedResponse {
    pub fn new(status: u16, headers: Headers, body: Bytes) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn body(&self) -> &Bytes {
        &self.body
    }

    pub async fn from_response(response: Response) -> crate::Result<Self> {
        let (status, headers, body) = response.deconstruct();
        let body = body.collect().await?;
        Ok(Self::new(status, headers, body))
    }

    pub fn json<T>(&self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        crate::json::from_json(&self.body)
    }
}

pub trait RawResponse {
    fn raw_response(self) -> Option<CollectedResponse>;
}
