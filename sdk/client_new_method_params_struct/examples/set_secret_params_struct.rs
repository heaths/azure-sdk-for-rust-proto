use azure_client_new_methods_params_struct::{
    Secret, SecretClient, SecretClientOptions, SecretProperties, SetSecretRequest,
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
        .set_secret(&SetSecretRequest::new("secret-name", "secret-value"))
        .await?;

    let secret: Secret = response.json().await?;
    println!("set {} version {}", secret.name, secret.version);

    // More complex client method call.
    let mut ctx = Context::default();
    ctx.insert("example".to_string());

    let mut request = SetSecretRequest::new("secret-name", "rotated-value");
    request.context = Some(ctx);
    request.properties = Some(SecretProperties { enabled: false });

    let response = client.set_secret(&request).await?;

    // Option 2: Implement async TryFrom<Response> for models, which customers can also do. Options are not mutually exclusive.
    // Note: async TryFrom<T> is still experimental but under consideration.
    let secret: Secret = response.json().await?;
    println!("set {} version {}", secret.name, secret.version);

    // Concurrent client method calls with same options.
    let mut ctx = Context::default();
    ctx.insert("example".to_string());

    let mut request_foo = SetSecretRequest::new("foo", "foo-value");
    request_foo.content_type = Some("text/plain".to_string());
    request_foo.context = Some(ctx.clone());

    let mut request_bar = SetSecretRequest::new("bar", "bar-value");
    request_bar.content_type = Some("text/plain".to_string());
    request_bar.context = Some(ctx.clone());

    let (_, _) = tokio::join!(
        client.set_secret(&request_foo),
        client.set_secret(&request_bar),
    );

    Ok(())
}
