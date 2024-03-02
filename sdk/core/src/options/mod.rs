mod retry;
mod transport;

#[cfg(feature = "builder")]
pub use builder::*;
pub use retry::*;
pub use transport::*;

use crate::policies::Policy;
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct ClientOptions {
    pub retry: RetryOptions,
    pub transport: TransportOptions,
    pub per_call_policies: Vec<Arc<dyn Policy>>,
    pub per_retry_policies: Vec<Arc<dyn Policy>>,
}

mod builder {
    use super::*;

    pub trait ClientBuilder {
        fn options(&mut self) -> &mut ClientOptions;

        fn retry(&mut self, retry: impl Into<RetryOptions>) -> &mut Self {
            self.options().retry = retry.into();
            self
        }

        fn transport(&mut self, transport: impl Into<TransportOptions>) -> &mut Self {
            self.options().transport = transport.into();
            self
        }
    }
}
