use azure_client_new_methods_params_context::{
    SecretClient, SecretClientOptions, SecretProperties,
};
use azure_core::{ClientOptions, ExponentialRetryOptions, RetryOptions};
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

    // Pass SecretProperties model.
    const NAME: &str = "secret-name";
    let properties = SecretProperties { enabled: true };
    let properties: SecretProperties = client
        .update_secret_properties(NAME, None::<String>, properties, None, None)
        .await?
        .try_into()?;
    println!("update {} properties: {properties:?}", NAME);

    // Pass bytes simulating a [u8] buffer from somewhere e.g., file or memory.
    let buf = r#"{"enabled": true}"#.as_bytes();
    let body: Vec<u8> = buf.to_vec();

    let properties: SecretProperties = client
        .update_secret_properties(NAME, None::<String>, body, None, None)
        .await?
        .try_into()?;
    println!("update {} properties: {properties:?}", NAME);

    // Pass str.
    let properties: SecretProperties = client
        .update_secret_properties(NAME, None::<String>, r#"{"enabled": true}"#, None, None)
        .await?
        .try_into()?;
    println!("update {} properties: {properties:?}", NAME);

    Ok(())
}
