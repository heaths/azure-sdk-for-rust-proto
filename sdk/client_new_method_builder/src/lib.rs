use azure_core::{
    policies::{ApiKeyAuthenticationPolicy, Policy},
    ClientOptions, Pipeline, Response, Result, TokenCredential, Url,
};
use std::sync::Arc;

mod models;
pub use models::*;

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

    pub fn set_secret<N, V>(&self, name: N, value: V) -> set_secret::SetSecretRequestBuilder
    where
        N: Into<String>,
        V: Into<String>,
    {
        set_secret::SetSecretRequestBuilder {
            client: self.clone(),
            name: name.into(),
            value: value.into(),
            content_type: None,
            properties: None,
            context: None,
        }
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

mod set_secret {
    use super::*;
    use azure_core::{Context, Request, Span};
    use futures::future::BoxFuture;

    #[derive(Clone, Debug)]
    pub struct SetSecretRequestBuilder {
        pub(crate) client: SecretClient,
        pub(crate) name: String,
        pub(crate) value: String,
        pub(crate) content_type: Option<String>,
        pub(crate) properties: Option<SecretProperties>,
        pub(crate) context: Option<Context>,
    }

    impl SetSecretRequestBuilder {
        pub fn with_content_type(&mut self, content_type: impl Into<String>) -> &mut Self {
            self.content_type = Some(content_type.into());
            self
        }

        pub fn with_context(&mut self, context: Context) -> &mut Self {
            self.context = Some(context);
            self
        }

        pub fn with_properties(&mut self, properties: SecretProperties) -> &mut Self {
            self.properties = Some(properties);
            self
        }

        pub fn send(&self) -> BoxFuture<'static, Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let mut ctx = Context::default();
                    ctx.insert(Span::from("SecretClient::set_secret"));

                    let mut url = this.client.endpoint.clone();
                    url.set_path(&format!("secrets/{}", this.name));

                    let body = SetSecretRequest {
                        value: this.value,
                        properties: this.properties,
                        ..Default::default()
                    };
                    let mut request = Request::new(url, "GET");
                    request.set_json(&body)?;

                    this.client.pipeline.send(&mut ctx, &mut request).await
                }
            })
        }
    }
}
