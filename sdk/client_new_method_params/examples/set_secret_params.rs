use azure_client_new_methods_params::{
    SecretClient, SecretClientOptions, SecretProperties, SetSecretOptions,
};
use azure_core::{
    ClientOptions, Context, ExponentialRetryOptions, RawResponse as _, RetryOptions, REQUEST_ID,
};
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
    let secret = client
        .set_secret("secret-name", "secret-value", None)
        .await?;

    println!("set {} version {}", secret.name, secret.version);

    // More complex client method call.
    let mut ctx = Context::default();
    ctx.insert("example".to_string());

    let secret = client
        .set_secret(
            "secret-name",
            "rotated-value",
            Some(SetSecretOptions {
                context: Some(ctx),
                properties: Some(SecretProperties { enabled: false }),
                if_none_match: secret.etag,
                ..Default::default()
            }),
        )
        .await?;
    println!("set {} version {}", secret.name, secret.version);

    // Get the x-ms-request-id from the raw response headers.
    if let Some(response) = secret.raw_response() {
        if let Some(request_id) = response.headers().get_optional_str(&REQUEST_ID) {
            println!("x-ms-request-id: {request_id}");
        }
    }

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
