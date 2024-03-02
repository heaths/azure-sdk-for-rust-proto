mod retry;
mod transport;

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
