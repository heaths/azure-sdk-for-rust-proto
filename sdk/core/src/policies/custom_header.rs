use crate::{
    context::Context,
    headers::{HeaderName, HeaderValue},
    policies::{Policy, PolicyResult},
    request::Request,
};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct CustomHeaderPolicy {
    name: String,
    value: String,
}

impl CustomHeaderPolicy {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[async_trait::async_trait]
impl Policy for CustomHeaderPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        request.insert_header::<HeaderName, HeaderValue>(
            self.name.clone().into(),
            self.value.clone().into(),
        );
        next[0].send(ctx, request, &next[1..]).await
    }
}
