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

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum RetryMode {
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

impl From<RetryOptions> for RetryMode {
    fn from(options: RetryOptions) -> Self {
        options.mode
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExponentialRetryOptions {}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FixedRetryOptions {}
