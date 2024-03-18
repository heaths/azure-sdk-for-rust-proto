#![doc = include_str!("../README.md")]

mod models;

use azure_core::{
    policies::{ApiKeyAuthenticationPolicy, Policy},
    ClientOptions, Context, Pipeline, Request, Response, Result, Span, TokenCredential, Url,
};
pub use models::*;
use std::{collections::HashMap, sync::Arc};

// NOTE: Implementing automock forces 'static lifetime on arguments.
#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait SecretClientMethods {
    fn endpoint(&self) -> &Url {
        unimplemented!()
    }

    async fn set_secret<N, V>(
        &self,
        _name: N,
        _value: V,
        _options: Option<SetSecretOptions>,
    ) -> azure_core::Result<Response>
    where
        N: Into<String> + Send + 'static,
        V: Into<String> + Send + 'static,
    {
        unimplemented!()
    }
}

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
}

#[async_trait::async_trait]
impl SecretClientMethods for SecretClient {
    fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    #[allow(unused_variables)]
    async fn set_secret<N, V>(
        &self,
        name: N,
        value: V,
        options: Option<SetSecretOptions>,
    ) -> azure_core::Result<Response>
    where
        N: Into<String> + Send,
        V: Into<String> + Send,
    {
        let options = options.unwrap_or_default();

        let mut ctx = options.context.unwrap_or_default();
        ctx.insert(Span::from("SecretClient::set_secret"));

        let mut url = self.endpoint.clone();
        url.set_path(&format!("secrets/{}", name.into()));

        let mut request = Request::new(url, "GET");
        request.set_json(&SetSecretRequest {
            value: value.into(),
            properties: options.properties,
            ..Default::default()
        })?;

        self.pipeline.send(&mut ctx, &mut request).await
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::{json::to_json, stream::BytesStream, Headers};

    #[tokio::test]
    async fn test_set_secret() {
        let mut client = MockSecretClientMethods::new();
        client
            .expect_set_secret()
            .returning(|name: &str, value: &str, _| {
                let secret = Secret {
                    name: name.to_string(),
                    version: value.to_string(),
                    properties: SecretProperties { enabled: true },
                };
                let json: BytesStream = to_json(&secret).expect("serialize Secret").into();
                Ok(Response::new(200, Headers::default(), Box::pin(json)))
            });

        let response = client.set_secret("test-secret", "secret-value", None).await;
        let secret: Secret = response
            .expect("expected response")
            .json()
            .await
            .expect("expected Secret");
        assert!(secret.properties.enabled);
    }
}
