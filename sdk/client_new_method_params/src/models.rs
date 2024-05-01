use azure_core::{CollectedResponse, Etag, RawResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct Secret {
    // Allocated memory.
    pub name: String,
    pub version: String,
    #[serde(rename = "attributes")]
    pub properties: SecretProperties,
    #[serde(skip)]
    pub etag: Option<Etag>,

    // Allocated memory essentially duplicated above.
    #[serde(skip)]
    pub(crate) _raw_response: Option<CollectedResponse>,
}

impl RawResponse for Secret {
    fn raw_response(self) -> Option<CollectedResponse> {
        self._raw_response
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
