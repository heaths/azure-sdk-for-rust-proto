use crate::response::Response;
use azure_core::{Etag, ETAG};
use futures::executor::block_on;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct Secret {
    pub name: String,
    pub version: String,
    #[serde(rename = "attributes")]
    pub properties: SecretProperties,
    #[serde(skip)]
    pub etag: Option<Etag>,
}

impl TryFrom<Response<Secret>> for Secret {
    type Error = azure_core::Error;

    fn try_from(value: Response<Secret>) -> Result<Self, Self::Error> {
        let etag: Option<Etag> = value.headers().get_optional_str(&ETAG).map(Etag::from);

        let f = || value.into_body().json::<Secret>();
        let mut secret = block_on(f())?;

        secret.etag = etag;
        Ok(secret)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub(crate) struct SetSecretRequest {
    pub value: String,
    #[serde(rename = "attributes")]
    pub properties: Option<SecretProperties>,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SecretProperties {
    pub enabled: bool,
}
