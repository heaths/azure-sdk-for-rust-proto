use std::task::Poll;

use bytes::Bytes;
use futures::Stream;

#[derive(Clone, Debug)]
pub struct BytesStream {
    bytes: Bytes,
    bytes_read: usize,
}

impl BytesStream {
    pub fn new(bytes: impl Into<Bytes>) -> Self {
        Self {
            bytes: bytes.into(),
            bytes_read: 0,
        }
    }
}

impl From<Bytes> for BytesStream {
    fn from(bytes: Bytes) -> Self {
        Self::new(bytes)
    }
}

impl Stream for BytesStream {
    type Item = crate::Result<Bytes>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let self_mut = self.get_mut();

        if self_mut.bytes_read < self_mut.bytes.len() {
            let bytes_read = self_mut.bytes_read;
            self_mut.bytes_read = self_mut.bytes.len();
            Poll::Ready(Some(Ok(self_mut.bytes.slice(bytes_read..))))
        } else {
            Poll::Ready(None)
        }
    }
}
