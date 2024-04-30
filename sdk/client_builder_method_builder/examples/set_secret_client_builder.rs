use azure_client_builder_method_builder::{Secret, SecretClient, SecretProperties};
use azure_core::{ClientBuilder, Context, ContextExt, ExponentialRetryOptions, RetryOptions};
use azure_identity::DefaultAzureCredential;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = env::var("AZURE_KEYVAULT_URL")?;

    let credential = Arc::new(DefaultAzureCredential::default());
    let client = SecretClient::builder(endpoint, credential)? // Create a mutable builder.
        .with_api_version("7.4")
        .with_retry(RetryOptions::exponential(ExponentialRetryOptions::default()))
        .build(); // Creates an immutable client.

    // Simple client method call.
    let response = client
        .set_secret("secret-name", "secret-value")
        .send()
        .await?;

    let secret: Secret = response.json().await?;
    println!("set {} version {}", secret.name, secret.version);

    // More complex client method call.
    let ctx = Context::default().with_retry(RetryOptions::none());

    let response = client
        .set_secret("secret-name", "rotated-value")
        .with_context(ctx)
        .with_properties(SecretProperties { enabled: false })
        .send()
        .await?;

    let secret: Secret = response.json().await?;
    println!("set {} version {}", secret.name, secret.version);

    // Concurrent client method calls with same options.
    let ctx = Context::default().with_value("example".to_string());

    let (_, _) = tokio::join!(
        async {
            client
                .set_secret("foo", "foo-value")
                .with_content_type("text/plain")
                .with_context(ctx.clone())
                .send()
                .await
        },
        async {
            client
                .set_secret("bar", "bar-value")
                .with_content_type("text/plain")
                .with_context(ctx.clone())
                .send()
                .await
        },
    );

    Ok(())
}
