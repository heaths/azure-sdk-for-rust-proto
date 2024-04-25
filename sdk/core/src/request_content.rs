use crate::Body;
use bytes::Bytes;
use std::{marker::PhantomData, str::FromStr};

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

impl<T> PartialEq for RequestContent<T> {
    fn eq(&self, other: &Self) -> bool {
        self.body.eq(&other.body)
    }
}

impl<T> From<RequestContent<T>> for Body {
    fn from(content: RequestContent<T>) -> Self {
        Body::from(content.body)
    }
}

impl<T> TryFrom<Bytes> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: Bytes) -> crate::Result<Self> {
        Ok(Self {
            body,
            _phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<Vec<u8>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(value: Vec<u8>) -> crate::Result<Self> {
        Ok(Self {
            body: Bytes::from(value),
            _phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<&'static str> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: &'static str) -> crate::Result<Self> {
        let body = Bytes::from_static(body.as_bytes());
        Ok(Self {
            body,
            _phantom: PhantomData,
        })
    }
}

impl<T> FromStr for RequestContent<T> {
    type Err = crate::Error;
    fn from_str(body: &str) -> Result<Self, Self::Err> {
        let body: Bytes = Bytes::copy_from_slice(body.as_bytes());
        Ok(Self {
            body,
            _phantom: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    struct Expected {
        str: String,
        num: i32,
        b: bool,
    }

    impl TryFrom<Expected> for RequestContent<Expected> {
        type Error = crate::Error;
        fn try_from(value: Expected) -> crate::Result<Self> {
            Ok(RequestContent::from(serde_json::to_vec(&value)?))
        }
    }

    static EXPECTED: Lazy<RequestContent<Expected>> = Lazy::new(|| RequestContent {
        body: Bytes::from(r#"{"str":"test","num":1,"b":true}"#.to_string()),
        _phantom: PhantomData,
    });

    #[test]
    fn tryfrom_t() {
        let actual = Expected {
            str: "test".to_string(),
            num: 1,
            b: true,
        };
        assert_eq!(*EXPECTED, actual.try_into().unwrap());
    }

    #[test]
    fn tryfrom_bytes() {
        let actual = Bytes::from(r#"{"str":"test","num":1,"b":true}"#.to_string());
        assert_eq!(*EXPECTED, actual.try_into().unwrap());
    }

    #[test]
    fn tryfrom_vec() {
        let actual: Vec<u8> = r#"{"str":"test","num":1,"b":true}"#.bytes().collect();
        assert_eq!(*EXPECTED, actual.try_into().unwrap());
    }

    #[test]
    fn tryfrom_str() {
        let actual = r#"{"str":"test","num":1,"b":true}"#;
        assert_eq!(*EXPECTED, actual.try_into().unwrap());
    }

    #[test]
    fn fromstr_parse() {
        let actual: RequestContent<Expected> =
            r#"{"str":"test","num":1,"b":true}"#.parse().unwrap();
        assert_eq!(*EXPECTED, actual);
    }
}
