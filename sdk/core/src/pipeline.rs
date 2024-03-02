use std::sync::Arc;

use crate::{
    context::Context,
    options::ClientOptions,
    policies::{Policy, TransportPolicy},
    request::Request,
    response::Response,
};

#[derive(Clone, Debug)]
pub struct Pipeline {
    pipeline: Vec<Arc<dyn Policy>>,
}

impl Pipeline {
    pub fn new(
        _crate_name: Option<&'static str>,
        _crate_version: Option<&'static str>,
        options: &ClientOptions,
        per_call_policies: Vec<Arc<dyn Policy>>,
        per_retry_policies: Vec<Arc<dyn Policy>>,
    ) -> Self {
        let mut pipeline: Vec<Arc<dyn Policy>> = Vec::with_capacity(
            per_call_policies.len()
                + options.per_call_policies.len()
                + per_retry_policies.len()
                + options.per_retry_policies.len()
                + 1,
        );

        pipeline.extend_from_slice(&per_call_policies);
        pipeline.extend_from_slice(&options.per_call_policies);

        // TODO: Telemetry, custom headers, etc. policies.
        // TODO: Retry policy.

        pipeline.extend_from_slice(&per_retry_policies);
        pipeline.extend_from_slice(&options.per_retry_policies);

        let transport: Arc<dyn Policy> = Arc::new(TransportPolicy::new(options.transport.clone()));
        pipeline.push(transport);

        Self { pipeline }
    }

    pub async fn send(&self, ctx: &Context, request: &mut Request) -> crate::Result<Response> {
        self.pipeline[0]
            .send(ctx, request, &self.pipeline[1..])
            .await
    }
}
