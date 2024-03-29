use crate::error::{Error, ErrorKind, ResultExt};
use std::str::FromStr;

pub trait AsHeaders {
    type Iter: Iterator<Item = (HeaderName, HeaderValue)>;
    fn as_headers(&self) -> Self::Iter;
}

impl<T> AsHeaders for T
where
    T: Header,
{
    type Iter = std::vec::IntoIter<(HeaderName, HeaderValue)>;

    fn as_headers(&self) -> Self::Iter {
        vec![(self.name(), self.value())].into_iter()
    }
}

impl<T> AsHeaders for Option<T>
where
    T: AsHeaders<Iter = std::vec::IntoIter<(HeaderName, HeaderValue)>>,
{
    type Iter = T::Iter;

    fn as_headers(&self) -> Self::Iter {
        match self {
            Some(h) => h.as_headers(),
            None => vec![].into_iter(),
        }
    }
}

pub trait Header {
    fn name(&self) -> HeaderName;
    fn value(&self) -> HeaderValue;
}

/// A collection of headers
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Headers(std::collections::HashMap<HeaderName, HeaderValue>);

impl Headers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_optional_string(&self, key: &HeaderName) -> Option<String> {
        self.get_as(key).ok()
    }

    pub fn get_str(&self, key: &HeaderName) -> crate::Result<&str> {
        self.get_with(key, |s| crate::Result::Ok(s.as_str()))
    }

    pub fn get_optional_str(&self, key: &HeaderName) -> Option<&str> {
        self.get_str(key).ok()
    }

    pub fn get_as<V, E>(&self, key: &HeaderName) -> crate::Result<V>
    where
        V: FromStr<Err = E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        self.get_with(key, |s| s.as_str().parse())
    }

    pub fn get_optional_as<V, E>(&self, key: &HeaderName) -> crate::Result<Option<V>>
    where
        V: FromStr<Err = E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        self.get_optional_with(key, |s| s.as_str().parse())
    }

    pub fn get_with<'a, V, F, E>(&'a self, key: &HeaderName, parser: F) -> crate::Result<V>
    where
        F: FnOnce(&'a HeaderValue) -> Result<V, E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        self.get_optional_with(key, parser)?.ok_or_else(|| {
            Error::with_message(ErrorKind::DataConversion, || {
                format!("header not found {}", key.as_str())
            })
        })
    }

    pub fn get_optional_with<'a, V, F, E>(
        &'a self,
        key: &HeaderName,
        parser: F,
    ) -> crate::Result<Option<V>>
    where
        F: FnOnce(&'a HeaderValue) -> Result<V, E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        self.0
            .get(key)
            .map(|v: &HeaderValue| {
                parser(v).with_context(ErrorKind::DataConversion, || {
                    let ty = std::any::type_name::<V>();
                    format!("unable to parse header '{key:?}: {v:?}' into {ty}",)
                })
            })
            .transpose()
    }

    /// Insert a header name/value pair
    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<HeaderName>,
        V: Into<HeaderValue>,
    {
        self.0.insert(key.into(), value.into());
    }

    /// Add headers to the headers collection
    pub fn add<H>(&mut self, header: H)
    where
        H: AsHeaders,
    {
        for (key, value) in header.as_headers() {
            self.insert(key, value);
        }
    }

    /// Iterate over all the header name/value pairs
    pub fn iter(&self) -> impl Iterator<Item = (&HeaderName, &HeaderValue)> {
        self.0.iter()
    }
}

impl IntoIterator for Headers {
    type Item = (HeaderName, HeaderValue);

    type IntoIter = std::collections::hash_map::IntoIter<HeaderName, HeaderValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<std::collections::HashMap<HeaderName, HeaderValue>> for Headers {
    fn from(c: std::collections::HashMap<HeaderName, HeaderValue>) -> Self {
        Self(c)
    }
}

/// A header name
#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct HeaderName(std::borrow::Cow<'static, str>);

impl HeaderName {
    pub const fn from_static(s: &'static str) -> Self {
        ensure_no_uppercase(s);
        Self(std::borrow::Cow::Borrowed(s))
    }

    fn from_cow<C>(c: C) -> Self
    where
        C: Into<std::borrow::Cow<'static, str>>,
    {
        let c = c.into();
        assert!(
            c.chars().all(|c| c.is_lowercase() || !c.is_alphabetic()),
            "header names must be lowercase: {c}"
        );
        Self(c)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

const fn ensure_no_uppercase(s: &str) {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let byte = bytes[i];
        assert!(
            !(byte >= 65u8 && byte <= 90u8),
            "header names must not contain uppercase letters"
        );
        i += 1;
    }
}

impl From<&'static str> for HeaderName {
    fn from(s: &'static str) -> Self {
        Self::from_cow(s)
    }
}

impl From<String> for HeaderName {
    fn from(s: String) -> Self {
        Self::from_cow(s.to_lowercase())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeaderValue(std::borrow::Cow<'static, str>);

impl HeaderValue {
    pub const fn from_static(s: &'static str) -> Self {
        Self(std::borrow::Cow::Borrowed(s))
    }

    pub fn from_cow<C>(c: C) -> Self
    where
        C: Into<std::borrow::Cow<'static, str>>,
    {
        Self(c.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<&'static str> for HeaderValue {
    fn from(s: &'static str) -> Self {
        Self::from_cow(s)
    }
}

impl From<String> for HeaderValue {
    fn from(s: String) -> Self {
        Self::from_cow(s)
    }
}

impl From<&String> for HeaderValue {
    fn from(s: &String) -> Self {
        s.clone().into()
    }
}

pub const ACCEPT: HeaderName = HeaderName::from_static("accept");
pub const AUTHORIZATION: HeaderName = HeaderName::from_static("authorization");
pub const CLIENT_REQUEST_ID: HeaderName = HeaderName::from_static("x-ms-client-request-id");
pub const CONTENT_ENCODING: HeaderName = HeaderName::from_static("content-encoding");
pub const CONTENT_LENGTH: HeaderName = HeaderName::from_static("content-length");
pub const ETAG: HeaderName = HeaderName::from_static("etag");
pub const IF_MATCH: HeaderName = HeaderName::from_static("if-match");
pub const IF_MODIFIED_SINCE: HeaderName = HeaderName::from_static("if-modified-since");
pub const IF_NONE_MATCH: HeaderName = HeaderName::from_static("if-none-match");
pub const IF_UNMODIFIED_SINCE: HeaderName = HeaderName::from_static("if-unmodified-since");
pub const TAGS: HeaderName = HeaderName::from_static("x-ms-tags");
pub const USER_AGENT: HeaderName = HeaderName::from_static("user-agent");
pub const WWW_AUTHENTICATE: HeaderName = HeaderName::from_static("www-authenticate");
