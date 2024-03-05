use crate::{
    auth::TokenCredential,
    context::Context,
    policies::{Policy, PolicyResult},
    request::Request,
};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ApiKeyAuthenticationPolicy {
    credential: Arc<dyn TokenCredential>,
    scope: String,
}

impl ApiKeyAuthenticationPolicy {
    pub fn new(credential: Arc<dyn TokenCredential>, scope: String) -> Self {
        Self { credential, scope }
    }
}

#[async_trait::async_trait]
impl Policy for ApiKeyAuthenticationPolicy {
    async fn send(
        &self,
        ctx: &mut Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        next[0].send(ctx, request, &next[1..]).await
    }
}
