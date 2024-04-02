#![doc = include_str!("../README.md")]

mod models;
mod response;

use azure_core::{
    policies::{ApiKeyAuthenticationPolicy, Policy},
    ClientOptions, Context, Pipeline, Request, Result, Span, TokenCredential, Url,
};
pub use models::*;
pub use response::*;
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
        ctx: Option<&Context>,
    ) -> azure_core::Result<Response<Secret>>
    where
        N: Into<String>,
        V: Into<String>,
    {
        let mut ctx = ctx.map_or_else(Context::default, Context::with_context);
        ctx.insert(Span::from("SecretClient::set_secret"));

        let mut url = self.endpoint.clone();
        url.set_path(&format!("secrets/{}", name.into()));

        let mut request = Request::new(url, "GET");
        request.set_json(&SetSecretRequest {
            value: value.into(),
            properties: options.and_then(|v| v.properties),
            ..Default::default()
        })?;

        self.pipeline
            .send(&mut ctx, &mut request)
            .await
            .map(Into::<Response<Secret>>::into)
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
