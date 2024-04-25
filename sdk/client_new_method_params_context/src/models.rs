use crate::response::Response;
use azure_core::{DateTime, RequestContent};
use futures::executor::block_on;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct Secret {
    pub name: String,
    pub version: String,
    #[serde(rename = "attributes")]
    pub properties: SecretProperties,
}

impl TryFrom<Response<Secret>> for Secret {
    type Error = azure_core::Error;

    fn try_from(value: Response<Secret>) -> Result<Self, Self::Error> {
        let f = || value.into_body().json::<Secret>();
        block_on(f())
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub(crate) struct SetSecretRequest {
    pub value: String,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecretProperties>,
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, String>>,

    #[serde(rename = "created", skip_serializing)]
    pub created_on: Option<DateTime>,
    #[serde(rename = "updated", skip_serializing)]
    pub updated_on: Option<DateTime>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SecretProperties {
    pub enabled: bool,
}

impl TryFrom<SecretProperties> for RequestContent<SecretProperties> {
    type Error = azure_core::Error;
    fn try_from(value: SecretProperties) -> azure_core::Result<Self> {
        Ok(RequestContent::from(serde_json::to_vec(&value)?))
    }
}

impl TryFrom<Response<SecretProperties>> for SecretProperties {
    type Error = azure_core::Error;

    fn try_from(value: Response<SecretProperties>) -> Result<Self, Self::Error> {
        let f = || value.into_body().json();
        block_on(f())
    }
}
