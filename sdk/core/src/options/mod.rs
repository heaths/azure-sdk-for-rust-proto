mod retry;
mod transport;

#[cfg(feature = "builder")]
pub use azure_core_macros::client_builder;
#[cfg(feature = "builder")]
pub use builder::*;

pub use retry::*;
pub use transport::*;

use crate::{context::Context, policies::Policy};
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct ClientOptions {
    // NOTE: These fields would not be public if using builders.
    pub context: Context,
    pub retry: RetryOptions,
    pub transport: TransportOptions,
    pub per_call_policies: Vec<Arc<dyn Policy>>,
    pub per_retry_policies: Vec<Arc<dyn Policy>>,
}

#[derive(Clone, Debug, Default)]
pub struct ClientMethodOptions {
    context: Context,
    retry: RetryOptions,
    per_call_policies: Vec<Arc<dyn Policy>>,
    per_retry_policies: Vec<Arc<dyn Policy>>,
}

impl ClientMethodOptions {
    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn retry(&self) -> &RetryOptions {
        &self.retry
    }

    pub fn per_call_policies(&self) -> &Vec<Arc<dyn Policy>> {
        &self.per_call_policies
    }

    pub fn per_retry_policies(&self) -> &Vec<Arc<dyn Policy>> {
        &self.per_retry_policies
    }
}

#[cfg(feature = "builder")]
mod builder {
    use super::*;

    pub trait ClientBuilder {
        fn options(&mut self) -> &mut ClientOptions;

        fn with_context(&mut self, context: Context) -> &mut Self {
            self.options().context = context;
            self
        }

        fn with_retry(&mut self, retry: impl Into<RetryOptions>) -> &mut Self {
            self.options().retry = retry.into();
            self
        }

        fn with_transport(&mut self, transport: impl Into<TransportOptions>) -> &mut Self {
            self.options().transport = transport.into();
            self
        }
    }

    pub trait ClientOptionsBuilder {
        fn options(&self) -> &ClientOptions;

        fn options_mut(&mut self) -> &mut ClientOptions;

        fn context(&self) -> &Context {
            &self.options().context
        }

        fn with_context(&mut self, context: Context) -> &mut Self {
            self.options_mut().context = context;
            self
        }

        fn retry(&self) -> &RetryOptions {
            &self.options().retry
        }

        fn with_retry(&mut self, retry: impl Into<RetryOptions>) -> &mut Self {
            self.options_mut().retry = retry.into();
            self
        }

        fn transport(&self) -> &TransportOptions {
            &self.options().transport
        }

        fn with_transport(&mut self, transport: impl Into<TransportOptions>) -> &mut Self {
            self.options_mut().transport = transport.into();
            self
        }

        fn per_call_policies(&self) -> &Vec<Arc<dyn Policy>> {
            &self.options().per_call_policies
        }

        fn with_per_call_policies(
            &mut self,
            per_call_policies: impl Into<Vec<Arc<dyn Policy>>>,
        ) -> &mut Self {
            self.options_mut()
                .per_call_policies
                .extend(per_call_policies.into());
            self
        }

        fn per_retry_policies(&self) -> &Vec<Arc<dyn Policy>> {
            &self.options().per_retry_policies
        }

        fn with_per_retry_policies(
            &mut self,
            per_retry_policies: impl Into<Vec<Arc<dyn Policy>>>,
        ) -> &mut Self {
            self.options_mut()
                .per_retry_policies
                .extend(per_retry_policies.into());
            self
        }
    }

    pub trait ClientMethodOptionsBuilder {
        fn options(&self) -> &ClientMethodOptions;

        fn options_mut(&mut self) -> &mut ClientMethodOptions;

        fn context(&self) -> &Context {
            &self.options().context
        }

        fn with_context(&mut self, context: Context) -> &mut Self {
            self.options_mut().context = context;
            self
        }

        fn retry(&self) -> &RetryOptions {
            &self.options().retry
        }

        fn with_retry(&mut self, retry: impl Into<RetryOptions>) -> &mut Self {
            self.options_mut().retry = retry.into();
            self
        }

        fn per_call_policies(&self) -> &Vec<Arc<dyn Policy>> {
            &self.options().per_call_policies
        }

        fn with_per_call_policies(
            &mut self,
            per_call_policies: impl Into<Vec<Arc<dyn Policy>>>,
        ) -> &mut Self {
            self.options_mut()
                .per_call_policies
                .extend(per_call_policies.into());
            self
        }

        fn per_retry_policies(&self) -> &Vec<Arc<dyn Policy>> {
            &self.options().per_retry_policies
        }

        fn with_per_retry_policies(
            &mut self,
            per_retry_policies: impl Into<Vec<Arc<dyn Policy>>>,
        ) -> &mut Self {
            self.options_mut()
                .per_retry_policies
                .extend(per_retry_policies.into());
            self
        }
    }
}
