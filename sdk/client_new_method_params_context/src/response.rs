use azure_core::{Headers, ResponseBody};
use std::marker::PhantomData;

// NOTE: This would be implemented in azure_core without the `From<azure_core::Response>`.
pub struct Response<T> {
    status: u16,
    headers: Headers,
    body: ResponseBody,
    _phantom: PhantomData<T>,
}

impl<T> Response<T> {
    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn into_body(self) -> ResponseBody {
        self.body
    }
}

impl<T> From<azure_core::Response> for Response<T> {
    fn from(value: azure_core::Response) -> Self {
        Self {
            status: value.status(),
            headers: value.headers().clone(),
            body: value.into_body(),
            _phantom: PhantomData,
        }
    }
}
