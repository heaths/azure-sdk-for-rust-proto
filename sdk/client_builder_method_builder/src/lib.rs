use azure_core::{
    policies::{ApiKeyAuthenticationPolicy, Policy},
    ClientBuilder, ClientOptions, Pipeline, Response, Result, TokenCredential, Url,
};
use std::sync::Arc;

mod models;
pub use models::*;

pub const DEFAULT_API_VERSION: &str = "7.5";

#[derive(Clone, Debug)]
pub struct SecretClientBuilder {
    endpoint: Url,
    credential: Arc<dyn TokenCredential>,
    api_version: Option<String>,
    scopes: Option<Vec<String>>,
    options: ClientOptions,
}

impl SecretClientBuilder {
    pub fn new(endpoint: impl AsRef<str>, credential: Arc<dyn TokenCredential>) -> Result<Self> {
        Ok(Self {
            endpoint: Url::parse(endpoint.as_ref())?,
            credential: credential.clone(),
            api_version: None,
            scopes: None,
            options: ClientOptions::default(),
        })
    }

    pub fn api_version(&mut self, api_version: impl Into<String>) -> &mut Self {
        self.api_version = Some(api_version.into());
        self
    }

    pub fn scopes(&mut self, scopes: &[&str]) -> &mut Self {
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
        per_retry_policies.extend_from_slice(&self.options.per_retry_policies);

        SecretClient {
            endpoint,
            pipeline: Pipeline::new(
                option_env!("CARGO_PKG_NAME"),
                option_env!("CARGO_PKG_VERSION"),
                &self.options,
                self.options.per_call_policies.clone(),
                per_retry_policies,
            ),
        }
    }
}

impl ClientBuilder for SecretClientBuilder {
    fn options(&mut self) -> &mut ClientOptions {
        &mut self.options
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
            properties: None,
        }
    }
}

mod set_secret {
    use super::*;
    use azure_core::{Context, Request};
    use futures::future::BoxFuture;

    #[derive(Clone, Debug)]
    pub struct SetSecretRequestBuilder {
        pub(crate) client: SecretClient,
        pub(crate) name: String,
        pub(crate) value: String,
        pub(crate) properties: Option<SecretProperties>,
    }

    impl SetSecretRequestBuilder {
        pub fn properties(&mut self, properties: SecretProperties) -> &mut Self {
            self.properties = Some(properties);
            self
        }

        pub fn send(&self) -> BoxFuture<'static, Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let mut url = this.client.endpoint.clone();
                    url.set_path(&format!("secrets/{}", this.name));

                    let body = SetSecretRequest {
                        value: this.value,
                        properties: this.properties,
                        ..Default::default()
                    };
                    let mut request = Request::new(url, "GET");
                    request.set_json(&body)?;

                    this.client
                        .pipeline
                        .send(&mut Context::default(), &mut request)
                        .await
                }
            })
        }
    }
}
