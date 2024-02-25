use crate::{stream::BytesStream, Headers, Response};
use bytes::Bytes;

#[derive(Clone, Debug)]
pub struct TransportOptions {
    inner: TransportOptionsImpl,
}

#[derive(Clone, Debug)]
enum TransportOptionsImpl {
    Bytes(bytes::Bytes),
}

impl TransportOptions {
    pub fn new(response_body: impl Into<Bytes>) -> Self {
        let inner = TransportOptionsImpl::Bytes(response_body.into());
        Self { inner }
    }

    pub async fn send(
        &self,
        _ctx: &crate::Context,
        _request: &mut crate::Request,
    ) -> crate::Result<crate::Response> {
        use TransportOptionsImpl as I;
        match &self.inner {
            I::Bytes(bytes) => {
                let response = Response::new(
                    200,
                    Headers::default(),
                    Box::pin(BytesStream::new(bytes.clone())),
                );
                Ok(response)
            }
        }
    }
}

impl Default for TransportOptions {
    fn default() -> Self {
        Self {
            inner: TransportOptionsImpl::Bytes(Bytes::new()),
        }
    }
}
