use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Etag(String);

impl Etag {
    pub fn is_weak(&self) -> bool {
        self.0.starts_with("W/")
    }
}

impl<T> From<T> for Etag
where
    T: Into<String>,
{
    fn from(t: T) -> Self {
        Self(t.into())
    }
}

impl AsRef<str> for Etag {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl FromStr for Etag {
    type Err = crate::error::Error;
    fn from_str(s: &str) -> crate::Result<Self> {
        Ok(Self(s.into()))
    }
}

impl fmt::Display for Etag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{}"#, self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weak() {
        let etag: Etag = "W/abcd1234".into();
        assert!(etag.is_weak());
    }

    #[test]
    fn strong() {
        let etag: Etag = "abcd1234".into();
        assert!(!etag.is_weak());
    }

    #[test]
    fn contains_quotes() {
        let etag: Etag = "abcd1234".into();
        assert_eq!(r#"abcd1234"#, etag.to_string());
    }
}
