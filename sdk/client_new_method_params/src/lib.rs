#![doc = include_str!("../README.md")]

mod models;

use azure_core::{
    policies::{ApiKeyAuthenticationPolicy, Policy},
    ClientOptions, Context, Pipeline, Request, Response, Result, TokenCredential, Url,
};
pub use models::*;
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone)]
pub struct SecretClient {
    endpoint: Url,
    pipeline: Pipeline,
}

impl SecretClient {
    pub fn new(
        endpoint: impl AsRef<str>,
        credential: Arc<dyn TokenCredential>,
        options: Option<SecretClientOptions>,
    ) -> Result<Self> {
        let options = options.unwrap_or_default();
        let mut endpoint = Url::parse(endpoint.as_ref())?;
        endpoint
            .query_pairs_mut()
            .clear()
            .append_pair("api-version", &options.api_version);

        let auth_policy: Arc<dyn Policy> = Arc::new(ApiKeyAuthenticationPolicy::new(
            credential.clone(),
            "https://vault.azure.net/.default".to_string(),
        ));
        let per_retry_policies = vec![auth_policy];

        Ok(Self {
            endpoint,
            pipeline: Pipeline::new(
                option_env!("CARGO_PKG_NAME"),
                option_env!("CARGO_PKG_VERSION"),
                &options.options,
                Vec::default(),
                per_retry_policies,
            ),
        })
    }

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    #[allow(unused_variables)]
    pub async fn set_secret<N, V>(
        &self,
        name: N,
        value: V,
        options: Option<SetSecretOptions>,
    ) -> azure_core::Result<Response>
    where
        N: Into<String>,
        V: Into<String>,
    {
        let mut url = self.endpoint.clone();
        url.set_path(&format!("secrets/{}", name.into()));

        let mut request = Request::new(url, "GET");
        request.set_json(&SetSecretRequest {
            value: value.into(),
            ..Default::default()
        })?;

        self.pipeline.send(&Context::default(), &mut request).await
    }
}

#[derive(Debug, Clone)]
pub struct SecretClientOptions {
    pub api_version: String,
    pub options: ClientOptions,
}

impl Default for SecretClientOptions {
    fn default() -> Self {
        Self {
            api_version: "7.5".to_string(),
            options: ClientOptions::default(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SetSecretOptions {
    pub properties: Option<SecretProperties>,
    pub content_type: Option<String>,
    pub tags: Option<HashMap<String, String>>,
}
