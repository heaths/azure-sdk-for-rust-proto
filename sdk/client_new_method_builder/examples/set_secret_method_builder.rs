use azure_client_new_method_builder::{
    Secret, SecretClient, SecretClientOptions, SecretProperties,
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
        .set_secret("secret-name", "secret-value")
        .send()
        .await?;

    let secret: Secret = response.json().await?;
    println!("set {} version {}", secret.name, secret.version);

    // More complex client method call.
    let mut ctx = Context::default();
    ctx.insert("example".to_string());

    let response = client
        .set_secret("secret-name", "rotated-value")
        .with_context(ctx)
        .with_properties(SecretProperties { enabled: false })
        .send()
        .await?;

    let secret: Secret = response.json().await?;
    println!("set {} version {}", secret.name, secret.version);

    // Concurrent client method calls with same options.
    let mut ctx = Context::default();
    ctx.insert("example".to_string());

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
