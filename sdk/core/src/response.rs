use crate::headers::Headers;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use serde::de::DeserializeOwned;
use std::pin::Pin;

pub(crate) type PinnedStream = Pin<Box<dyn Stream<Item = crate::Result<Bytes>> + Send + Sync>>;

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

pub struct ResponseBody(PinnedStream);

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
