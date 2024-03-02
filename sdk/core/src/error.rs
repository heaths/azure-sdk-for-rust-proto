use std::{
    borrow::Cow,
    fmt::{Debug, Display},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    HttpResponse {
        status: u16,
        error_code: Option<String>,
    },
    Io,
    DataConversion,
    Credential,
    Other,
}

impl ErrorKind {
    pub fn into_error(self) -> Error {
        Error {
            context: Context::Simple(self),
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::HttpResponse { status, error_code } => {
                write!(
                    f,
                    "HttpResponse({}, {})",
                    status,
                    error_code.as_deref().unwrap_or("unknown")
                )
            }
            ErrorKind::Credential => write!(f, "Credential"),
            ErrorKind::DataConversion => write!(f, "DataConversion"),
            ErrorKind::Io => write!(f, "Io"),
            ErrorKind::Other => write!(f, "Other"),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    context: Context,
}

impl Error {
    pub fn new<E>(kind: ErrorKind, error: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self {
            context: Context::Custom(Custom {
                kind,
                error: error.into(),
            }),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        match &self.context {
            Context::Simple(kind)
            | Context::Message { kind, .. }
            | Context::Custom(Custom { kind, .. })
            | Context::Full(Custom { kind, .. }, _) => kind,
        }
    }

    #[must_use]
    pub fn message<C>(kind: ErrorKind, message: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        Self {
            context: Context::Message {
                kind,
                message: message.into(),
            },
        }
    }

    #[must_use]
    pub fn with_message<F, C>(kind: ErrorKind, message: F) -> Self
    where
        Self: Sized,
        F: FnOnce() -> C,
        C: Into<Cow<'static, str>>,
    {
        Self {
            context: Context::Message {
                kind,
                message: message().into(),
            },
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.context {
            Context::Simple(kind) => write!(f, "{kind}"),
            Context::Message { message, .. } => write!(f, "{message}"),
            Context::Custom(Custom { error, .. }) => write!(f, "{error}"),
            Context::Full(_, message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.context {
            Context::Custom(Custom { error, .. }) | Context::Full(Custom { error, .. }, _) => {
                Some(&**error)
            }
            _ => None,
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self {
            context: Context::Simple(kind),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::new(ErrorKind::Io, error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::new(ErrorKind::DataConversion, error)
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Self::new(ErrorKind::DataConversion, error)
    }
}

#[derive(Debug)]
enum Context {
    Simple(ErrorKind),
    Message {
        kind: ErrorKind,
        message: Cow<'static, str>,
    },
    Custom(Custom),
    Full(Custom, Cow<'static, str>),
}

#[derive(Debug)]
struct Custom {
    kind: ErrorKind,
    error: Box<dyn std::error::Error + Send + Sync>,
}

pub trait ResultExt<T>: private::Sealed {
    fn map_kind(self, kind: ErrorKind) -> Result<T>
    where
        Self: Sized;
    fn context<C>(self, kind: ErrorKind, message: C) -> Result<T>
    where
        Self: Sized,
        C: Into<Cow<'static, str>>;
    fn with_context<F, C>(self, kind: ErrorKind, f: F) -> Result<T>
    where
        Self: Sized,
        F: FnOnce() -> C,
        C: Into<Cow<'static, str>>;
}

mod private {
    pub trait Sealed {}
    impl<T, E> Sealed for std::result::Result<T, E> where E: std::error::Error + Send + Sync + 'static {}
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn map_kind(self, kind: ErrorKind) -> Result<T>
    where
        Self: Sized,
    {
        self.map_err(|e| Error::new(kind, e))
    }

    fn context<C>(self, kind: ErrorKind, message: C) -> Result<T>
    where
        Self: Sized,
        C: Into<Cow<'static, str>>,
    {
        self.map_err(|e| Error {
            context: Context::Full(
                Custom {
                    error: Box::new(e),
                    kind,
                },
                message.into(),
            ),
        })
    }

    fn with_context<F, C>(self, kind: ErrorKind, f: F) -> Result<T>
    where
        Self: Sized,
        F: FnOnce() -> C,
        C: Into<Cow<'static, str>>,
    {
        self.context(kind, f())
    }
}
