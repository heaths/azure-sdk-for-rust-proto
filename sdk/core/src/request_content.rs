use bytes::Bytes;
use std::marker::PhantomData;

use crate::Body;

#[derive(Debug, Clone)]
pub struct RequestContent<T> {
    body: Bytes,
    _phantom: PhantomData<T>,
}

impl<T> RequestContent<T> {
    pub fn body(&self) -> &Bytes {
        &self.body
    }

    pub fn from(bytes: Vec<u8>) -> Self {
        Self {
            body: Bytes::from(bytes),
            _phantom: PhantomData,
        }
    }
}

impl<T> From<RequestContent<T>> for Body {
    fn from(content: RequestContent<T>) -> Self {
        Body::from(content.body)
    }
}

impl<T> TryFrom<Bytes> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: Bytes) -> Result<Self, Self::Error> {
        Ok(Self {
            body,
            _phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<Vec<u8>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self {
            body: Bytes::from(value),
            _phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<&'static str> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: &'static str) -> Result<Self, Self::Error> {
        let body = Bytes::from_static(body.as_bytes());
        Ok(Self {
            body,
            _phantom: PhantomData,
        })
    }
}
