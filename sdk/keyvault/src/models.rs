use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct Secret {
    pub name: String,
    pub version: String,
    #[serde(rename = "attributes")]
    pub properties: SecretProperties,
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
