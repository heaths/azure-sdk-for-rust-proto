mod transport;

pub use transport::*;

#[derive(Clone, Debug, Default)]
pub struct ClientOptions {
    pub retry: RetryOptions,
    pub transport: TransportOptions,
}

#[derive(Clone, Debug, Default)]
pub struct RetryOptions {
    mode: RetryMode,
}

impl RetryOptions {
    pub fn exponential(options: ExponentialRetryOptions) -> Self {
        Self {
            mode: RetryMode::Exponential(options),
        }
    }

    pub fn fixed(options: FixedRetryOptions) -> Self {
        Self {
            mode: RetryMode::Fixed(options),
        }
    }

    pub fn none() -> Self {
        Self {
            mode: RetryMode::None,
        }
    }
}

#[derive(Clone, Debug)]
enum RetryMode {
    Exponential(ExponentialRetryOptions),
    Fixed(FixedRetryOptions),
    // Custom(Arc<dyn Policy>),
    None,
}

impl Default for RetryMode {
    fn default() -> Self {
        RetryMode::Exponential(ExponentialRetryOptions::default())
    }
}

#[derive(Clone, Debug, Default)]
pub struct ExponentialRetryOptions {}

#[derive(Clone, Debug, Default)]
pub struct FixedRetryOptions {}
