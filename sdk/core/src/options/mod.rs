mod retry;
mod transport;

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

    #[macro_export]
    macro_rules! client_options_builder {
        ($($field:ident).*) => {
            pub fn context(&self) -> &$crate::Context {
                &self.$($field).*.context
            }

            pub fn with_context(&mut self, context: $crate::Context) -> &mut Self {
                self.$($field).*.context = context;
                self
            }

            pub fn retry(&self) -> &$crate::RetryOptions {
                &self.$($field).*.retry
            }

            pub fn with_retry(
                &mut self,
                retry: impl ::std::convert::Into<$crate::RetryOptions>,
            ) -> &mut Self {
                self.$($field).*.retry = retry.into();
                self
            }

            pub fn transport(&self) -> &$crate::TransportOptions {
                &self.$($field).*.transport
            }

            pub fn with_transport(
                &mut self,
                transport: impl ::std::convert::Into<$crate::TransportOptions>,
            ) -> &mut Self {
                self.$($field).*.transport = transport.into();
                self
            }

            pub fn per_call_policies(&self) -> &::std::vec::Vec<::std::sync::Arc<dyn $crate::policies::Policy>> {
                &self.$($field).*.per_call_policies
            }

            pub fn with_per_call_policies(
                &mut self,
                per_call_policies: impl ::std::convert::Into<
                    ::std::vec::Vec<::std::sync::Arc<dyn $crate::policies::Policy>>,
                >,
            ) -> &mut Self {
                self.$($field).*
                    .per_call_policies
                    .extend(per_call_policies.into());
                self
            }

            pub fn per_retry_policies(&self) -> &::std::vec::Vec<::std::sync::Arc<dyn $crate::policies::Policy>> {
                &self.$($field).*.per_retry_policies
            }

            pub fn with_per_retry_policies(
                &mut self,
                per_retry_policies: impl ::std::convert::Into<
                    ::std::vec::Vec<::std::sync::Arc<dyn $crate::policies::Policy>>,
                >,
            ) -> &mut Self {
                self.$($field).*
                    .per_retry_policies
                    .extend(per_retry_policies.into());
                self
            }
        };
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
