use crate::{Context, Request, Response};
use std::sync::Arc;

mod transport;

pub use transport::*;

pub type PolicyResult = crate::error::Result<Response>;

#[async_trait::async_trait]
pub trait Policy: Send + Sync + std::fmt::Debug {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult;
}
