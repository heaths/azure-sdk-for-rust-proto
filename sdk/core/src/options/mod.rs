mod retry;
mod transport;

pub use builders::*;
pub use retry::*;
pub use transport::*;

use crate::context::Context;

#[derive(Clone, Debug, Default)]
pub struct ClientOptions {
    retry: RetryOptions,
    transport: TransportOptions,
}

impl ClientOptions {
    pub fn retry(&self) -> &RetryOptions {
        &self.retry
    }

    pub fn transport(&self) -> &TransportOptions {
        &self.transport
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientMethodOptions {
    context: Context,
}

impl ClientMethodOptions {
    pub fn context(&self) -> &Context {
        &self.context
    }
}

mod builders {
    use super::*;

    pub trait ClientOptionsBuilder {
        fn options(&self) -> &ClientOptions;

        fn options_mut(&mut self) -> &mut ClientOptions;

        fn with_retry(&mut self, retry: impl Into<RetryOptions>) -> &mut Self {
            self.options_mut().retry = retry.into();
            self
        }

        fn with_transport(&mut self, transport: impl Into<TransportOptions>) -> &mut Self {
            self.options_mut().transport = transport.into();
            self
        }
    }

    pub trait ClientMethodOptionsBuilder {
        fn options(&self) -> &ClientMethodOptions;

        fn options_mut(&mut self) -> &mut ClientMethodOptions;

        fn with_context(&mut self, context: Context) -> &mut Self {
            self.options_mut().context = context;
            self
        }
    }
}
