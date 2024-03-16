use azure_client_new_methods_params::{
    Secret, SecretClient, SecretClientMethods, SecretClientOptions, SecretProperties,
    SetSecretOptions,
};
use azure_core::{ClientOptions, Context, ExponentialRetryOptions, RetryOptions};
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

    // Simple client method call.
    let response = client
        .set_secret("secret-name", "secret-value", None)
        .await?;

    let secret: Secret = response.json().await?;
    println!("set {} version {}", secret.name, secret.version);

    // More complex client method call.
    let mut ctx = Context::default();
    ctx.insert("example".to_string());

    let response = client
        .set_secret(
            "secret-name",
            "rotated-value",
            Some(SetSecretOptions {
                context: Some(ctx),
                properties: Some(SecretProperties { enabled: false }),
                ..Default::default()
            }),
        )
        .await?;

    // Option 2: Implement async TryFrom<Response> for models, which customers can also do. Options are not mutually exclusive.
    // Note: async TryFrom<T> is still experimental but under consideration.
    let secret: Secret = response.json().await?;
    println!("set {} version {}", secret.name, secret.version);

    // Concurrent client method calls with same options.
    let mut ctx = Context::default();
    ctx.insert("example".to_string());

    let options = SetSecretOptions {
        content_type: Some("text/plain".to_string()),
        context: Some(ctx),
        ..Default::default()
    };

    let (_, _) = tokio::join!(
        client.set_secret("foo", "foo-value", Some(options.clone())),
        client.set_secret("bar", "bar-value", Some(options.clone())),
    );

    Ok(())
}
