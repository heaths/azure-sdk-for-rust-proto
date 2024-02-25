use std::sync::Arc;

use crate::{
    options::ClientOptions,
    policies::{Policy, TransportPolicy},
};

#[derive(Clone, Debug)]
pub struct Pipeline {
    pipeline: Vec<Arc<dyn Policy>>,
}

impl Pipeline {
    pub fn new(options: &ClientOptions) -> Self {
        let mut pipeline: Vec<Arc<dyn Policy>> = Vec::with_capacity(1);

        let transport: Arc<dyn Policy> = Arc::new(TransportPolicy::new(options.transport.clone()));
        pipeline.push(transport);

        Self { pipeline }
    }
}
