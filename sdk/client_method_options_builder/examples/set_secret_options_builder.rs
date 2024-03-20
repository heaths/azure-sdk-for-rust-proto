use azure_core::{
    ClientMethodOptionsBuilder, ClientOptionsBuilder, Context, ExponentialRetryOptions,
    RetryOptions,
};
use azure_identity::DefaultAzureCredential;
use client_method_options_builder::{
    Secret, SecretClient, SecretClientOptions, SecretProperties, SetSecretOptions,
};
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = env::var("AZURE_KEYVAULT_URL")?;

    let credential = Arc::new(DefaultAzureCredential::default());
    let options = SecretClientOptions::builder()
        .with_api_version("7.4")
        // BUGBUG: Not initially discoverable; rust-analyzer tells you want to import, but not immediately discoverable.
        .with_retry(RetryOptions::exponential(ExponentialRetryOptions {}))
        .build();
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

    let options = SetSecretOptions::builder()
        // BUGBUG: Not initially discoverable; rust-analyzer tells you want to import, but not immediately discoverable.
        .with_context(ctx)
        .with_properties(SecretProperties { enabled: false })
        .build();
    let response = client
        .set_secret("secret-name", "rotated-value", Some(options))
        .await?;

    // Option 2: Implement async TryFrom<Response> for models, which customers can also do. Options are not mutually exclusive.
    // Note: async TryFrom<T> is still experimental but under consideration.
    let secret: Secret = response.json().await?;
    println!("set {} version {}", secret.name, secret.version);

    // Concurrent client method calls with same options.
    let mut ctx = Context::default();
    ctx.insert("example".to_string());

    let options = SetSecretOptions::builder()
        .with_content_type("text/plain")
        // BUGBUG: Not initially discoverable; rust-analyzer tells you want to import, but not immediately discoverable.
        .with_context(ctx)
        .build();

    let (_, _) = tokio::join!(
        client.set_secret("foo", "foo-value", Some(options.clone())),
        client.set_secret("bar", "bar-value", Some(options.clone())),
    );

    Ok(())
}
