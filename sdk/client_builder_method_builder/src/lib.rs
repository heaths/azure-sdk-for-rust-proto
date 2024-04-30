use azure_core::{
    client_builder,
    policies::{ApiKeyAuthenticationPolicy, Policy},
    ClientOptions, Pipeline, Response, Result, TokenCredential, Url,
};
use std::sync::Arc;

mod models;
pub use models::*;

pub const DEFAULT_API_VERSION: &str = "7.5";

// NOTE: Attribute macro must listed before derive macros.
#[client_builder]
#[derive(Clone, Debug)]
pub struct SecretClientBuilder {
    endpoint: Url,
    credential: Arc<dyn TokenCredential>,
    api_version: Option<String>,
    scopes: Option<Vec<String>>,
}

impl SecretClientBuilder {
    pub fn new(endpoint: impl AsRef<str>, credential: Arc<dyn TokenCredential>) -> Result<Self> {
        Ok(Self {
            endpoint: Url::parse(endpoint.as_ref())?,
            credential: credential.clone(),
            api_version: None,
            scopes: None,
            // NOTE: I don't like this devex: that they have to use a field they didn't define. May as well use a derive attribute.
            __options: ClientOptions::default(),
        })
    }

    pub fn with_api_version(&mut self, api_version: impl Into<String>) -> &mut Self {
        self.api_version = Some(api_version.into());
        self
    }

    pub fn with_scopes(&mut self, scopes: &[&str]) -> &mut Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }

    pub fn build(&self) -> SecretClient {
        let mut endpoint = self.endpoint.clone();
        endpoint.query_pairs_mut().clear().append_pair(
            "api-version",
            self.api_version
                .as_ref()
                .map(|version| version.as_ref())
                .unwrap_or(DEFAULT_API_VERSION),
        );

        let auth_policy: Arc<dyn Policy> = Arc::new(ApiKeyAuthenticationPolicy::new(
            self.credential.clone(),
            "https://vault.azure.net/.default".to_string(),
        ));

        let mut per_retry_policies = vec![auth_policy];
        per_retry_policies.extend_from_slice(&self.__options.per_retry_policies);

        SecretClient {
            endpoint,
            pipeline: Pipeline::new(
                option_env!("CARGO_PKG_NAME"),
                option_env!("CARGO_PKG_VERSION"),
                &self.__options,
                self.__options.per_call_policies.clone(),
                per_retry_policies,
            ),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SecretClient {
    endpoint: Url,
    pipeline: Pipeline,
}

impl SecretClient {
    pub fn builder(
        endpoint: impl AsRef<str>,
        credential: Arc<dyn TokenCredential>,
    ) -> Result<SecretClientBuilder> {
        SecretClientBuilder::new(endpoint, credential)
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

        pub fn send(&self) -> BoxFuture<Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let mut ctx = this.context.unwrap_or_default();
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
