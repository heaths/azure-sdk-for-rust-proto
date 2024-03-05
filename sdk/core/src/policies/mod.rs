use crate::{context::Context, request::Request, response::Response};
use std::sync::Arc;

mod auth;
mod custom_header;
mod transport;

pub use auth::*;
pub use custom_header::*;
pub use transport::*;

pub type PolicyResult = crate::error::Result<Response>;

#[async_trait::async_trait]
pub trait Policy: Send + Sync + std::fmt::Debug {
    async fn send(
        &self,
        ctx: &mut Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult;
}
