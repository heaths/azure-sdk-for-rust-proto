mod models;

use azure_core::{
    policies::{ApiKeyAuthenticationPolicy, Policy},
    ClientMethodOptions, ClientOptions, Pipeline, Request, Response, Result, Span, TokenCredential,
    Url,
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
                &options.client_options,
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
        let options = options.unwrap_or_default();

        let mut ctx = options.client_method_options.context().clone();
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
    client_options: ClientOptions,
    api_version: String,
}

impl SecretClientOptions {
    pub fn api_version(&self) -> &str {
        self.api_version.as_str()
    }

    pub fn builder() -> builder::SecretClientOptionsBuilder {
        todo!()
    }
}

impl Default for SecretClientOptions {
    fn default() -> Self {
        Self {
            api_version: "7.5".to_string(),
            client_options: ClientOptions::default(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SetSecretOptions {
    properties: Option<SecretProperties>,
    content_type: Option<String>,
    tags: Option<HashMap<String, String>>,
    client_method_options: ClientMethodOptions,
}

impl SetSecretOptions {
    pub fn properties(&self) -> Option<&SecretProperties> {
        self.properties.as_ref()
    }

    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_ref().map(String::as_ref)
    }

    pub fn tags(&self) -> Option<&HashMap<String, String>> {
        self.tags.as_ref()
    }

    pub fn builder() -> builder::SetSecretOptionsBuilder {
        todo!()
    }
}

pub mod builder {
    use super::*;
    use azure_core::ClientMethodOptionsBuilder;

    #[derive(Default)]
    pub struct SecretClientOptionsBuilder {
        options: SecretClientOptions,
    }

    impl SecretClientOptionsBuilder {
        pub fn api_version(&self) -> &str {
            self.options.api_version.as_str()
        }

        pub fn with_api_version(&mut self, api_version: impl Into<String>) -> &mut Self {
            self.options.api_version = api_version.into();
            self
        }

        azure_core::client_options_builder!(options.client_options);

        pub fn build(&self) -> SecretClientOptions {
            self.options.clone()
        }
    }

    #[derive(Default)]
    pub struct SetSecretOptionsBuilder {
        options: SetSecretOptions,
    }

    impl SetSecretOptionsBuilder {
        pub fn properties(&self) -> Option<&SecretProperties> {
            self.options.properties.as_ref()
        }

        pub fn with_properties(&mut self, properties: SecretProperties) -> &mut Self {
            self.options.properties = Some(properties);
            self
        }

        pub fn content_type(&self) -> Option<&str> {
            self.options.content_type.as_ref().map(String::as_ref)
        }

        pub fn with_content_type(&mut self, content_type: impl Into<String>) -> &mut Self {
            self.options.content_type = Some(content_type.into());
            self
        }

        pub fn tags(&self) -> Option<&HashMap<String, String>> {
            self.options.tags.as_ref()
        }

        pub fn with_tags(&mut self, tags: HashMap<String, String>) -> &mut Self {
            self.options.tags = Some(tags);
            self
        }

        pub fn build(&self) -> SetSecretOptions {
            self.options.clone()
        }
    }

    impl ClientMethodOptionsBuilder for SetSecretOptionsBuilder {
        fn options(&self) -> &ClientMethodOptions {
            &self.options.client_method_options
        }

        fn options_mut(&mut self) -> &mut ClientMethodOptions {
            &mut self.options.client_method_options
        }
    }
}
