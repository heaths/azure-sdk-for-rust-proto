#![doc = include_str!("../README.md")]

mod models;

use azure_core::{
    policies::{ApiKeyAuthenticationPolicy, Policy},
    ClientOptions, CollectedResponse, Context, Etag, Pipeline, Request, Result, Span,
    TokenCredential, Url, ETAG, IF_MATCH, IF_NONE_MATCH,
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
    ) -> azure_core::Result<Secret>
    where
        N: Into<String>,
        V: Into<String>,
    {
        let options = options.unwrap_or_default();

        let mut ctx = options.context.unwrap_or_default();
        ctx.insert(Span::from("SecretClient::set_secret"));

        let mut url = self.endpoint.clone();
        url.set_path(&format!("secrets/{}", name.into()));

        let mut request = Request::new(url, "GET");
        // NOTE: This is done with strong types in existing code.
        // Shown here only as demonstration.
        if let Some(etag) = options.if_match {
            request.insert_header(IF_MATCH, etag.to_string());
        }
        if let Some(etag) = options.if_none_match {
            request.insert_header(IF_NONE_MATCH, etag.to_string());
        }
        request.set_json(&SetSecretRequest {
            value: value.into(),
            properties: options.properties,
            ..Default::default()
        })?;

        let response = self.pipeline.send(&mut ctx, &mut request).await?;
        let response = CollectedResponse::from_response(response).await?;
        let mut secret: Secret = response.json()?;
        secret.etag = response.headers().get_optional_str(&ETAG).map(Etag::from);
        secret._raw_response = Some(response);
        Ok(secret)
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
    pub context: Option<Context>,
    pub if_match: Option<Etag>,
    pub if_none_match: Option<Etag>,
}
