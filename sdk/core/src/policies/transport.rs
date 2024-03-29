use crate::{
    context::Context,
    options::TransportOptions,
    policies::{Policy, PolicyResult},
    request::Request,
};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct TransportPolicy {
    pub(crate) transport_options: TransportOptions,
}

impl TransportPolicy {
    pub fn new(transport_options: TransportOptions) -> Self {
        Self { transport_options }
    }
}

#[async_trait::async_trait]
impl Policy for TransportPolicy {
    async fn send(
        &self,
        ctx: &mut Context,
        request: &mut Request,
        _next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let response = { self.transport_options.send(ctx, request) };
        response.await
    }
}
