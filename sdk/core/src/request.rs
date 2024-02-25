use bytes::Bytes;
use serde::Serialize;
use url::Url;

use crate::{AsHeaders, Headers};

#[derive(Clone, Debug)]
pub enum Body {
    Bytes(bytes::Bytes),
}

impl Body {
    pub fn len(&self) -> usize {
        match self {
            Body::Bytes(bytes) => bytes.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<B> From<B> for Body
where
    B: Into<Bytes>,
{
    fn from(bytes: B) -> Self {
        Self::Bytes(bytes.into())
    }
}

#[derive(Clone, Debug)]
pub struct Request {
    pub(crate) url: Url,
    pub(crate) method: &'static str,
    pub(crate) headers: Headers,
    pub(crate) body: Body,
}

impl Request {
    pub fn new(url: Url, method: &'static str) -> Self {
        Self {
            url,
            method,
            headers: Headers::new(),
            body: Body::Bytes(bytes::Bytes::new()),
        }
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn path_and_query(&self) -> String {
        let mut result = self.url.path().to_owned();
        if let Some(query) = self.url.query() {
            result.push('?');
            result.push_str(query);
        }
        result
    }

    pub fn method(&self) -> &'static str {
        self.method
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn insert_header<K, V>(&mut self, key: K, value: V)
    where
        K: Into<crate::headers::HeaderName>,
        V: Into<crate::headers::HeaderValue>,
    {
        self.headers.insert(key, value);
    }

    pub fn insert_headers<T: AsHeaders>(&mut self, headers: &T) {
        for (name, value) in headers.as_headers() {
            self.insert_header(name, value)
        }
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn set_body(&mut self, body: impl Into<Body>) {
        self.body = body.into();
    }

    pub fn set_json<T>(&mut self, data: &T) -> crate::Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.set_body(crate::json::to_json(data)?);
        Ok(())
    }
}
