#![doc = include_str!("../README.md")]

mod models;
mod response;

use azure_core::{
    policies::{ApiKeyAuthenticationPolicy, Policy},
    ClientOptions, Context, Etag, Pipeline, Request, RequestContent, Result, Span, TokenCredential,
    Url, IF_MATCH, IF_NONE_MATCH,
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
        // NOTE: This is done with strong types in existing code.
        // Shown here only as demonstration.
        if let Some(ref options) = options {
            if let Some(etag) = &options.if_match {
                request.insert_header(IF_MATCH, etag.to_string());
            }
            if let Some(etag) = &options.if_none_match {
                request.insert_header(IF_NONE_MATCH, etag.to_string());
            }
        }
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

    #[allow(unused_variables)]
    pub async fn update_secret_properties<NAME, VERSION>(
        &self,
        name: NAME,
        version: Option<VERSION>,
        properties: impl TryInto<RequestContent<SecretProperties>, Error = azure_core::Error>,
        options: Option<UpdateSecretPropertiesOptions>,
        ctx: Option<&Context>,
    ) -> azure_core::Result<Response<SecretProperties>>
    where
        NAME: Into<String>,
        VERSION: Into<String>,
    {
        let mut ctx = ctx.map_or_else(Context::default, Context::with_context);
        ctx.insert(Span::from("SecretClient::update_secret_properties"));

        let mut url = self.endpoint.clone();
        url.set_path(&format!(
            "secrets/{}/{}",
            name.into(),
            version.map_or_else(String::new, |v| v.into())
        ));

        let mut request = Request::new(url, "PATCH");
        let body: RequestContent<SecretProperties> = properties.try_into()?;
        request.set_body(body);

        self.pipeline
            .send(&mut ctx, &mut request)
            .await
            .map(Into::<Response<SecretProperties>>::into)
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
    pub if_match: Option<Etag>,
    pub if_none_match: Option<Etag>,
}

#[derive(Clone, Debug, Default)]
pub struct UpdateSecretPropertiesOptions {}
