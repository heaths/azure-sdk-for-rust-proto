use azure_client_new_methods_params_context::{
    Secret, SecretClient, SecretClientOptions, SecretProperties, UpdateSecretPropertiesOptions,
};
use azure_core::{ClientOptions, ExponentialRetryOptions, HeadersExt, RetryOptions};
use azure_identity::DefaultAzureCredential;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = env::var("AZURE_KEYVAULT_URL")?;

    let credential = Arc::new(DefaultAzureCredential::default());
    let options = SecretClientOptions {
        api_version: "7.4".to_string(),
        options: ClientOptions {
            retry: RetryOptions::exponential(ExponentialRetryOptions {}),
            ..Default::default()
        },
    };
    let client = SecretClient::new(endpoint, credential, Some(options))?;

    const NAME: &str = "secret-name";
    let response = client.get_secret(NAME, None::<String>, None, None).await?;
    let etag = response.headers().etag().expect("missing etag");

    let secret: Secret = response.try_into()?;

    let mut properties = secret.properties;
    properties.enabled = false;

    let options = UpdateSecretPropertiesOptions {
        if_match: Some(etag),
        ..Default::default()
    };

    let properties: SecretProperties = client
        .update_secret_properties(NAME, None::<String>, properties, Some(options), None)
        .await?
        .try_into()?;
    println!("updated {NAME} properties: {properties:?}");

    Ok(())
}
