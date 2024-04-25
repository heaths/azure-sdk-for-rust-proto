use azure_client_new_methods_params_context::{
    Secret, SecretClient, SecretClientOptions, SecretProperties, SetSecretOptions,
};
use azure_core::{ClientOptions, Context, ErrorKind, ExponentialRetryOptions, RetryOptions};
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
    let secret: Secret = client
        .set_secret("secret-name", "secret-value", None, None)
        .await? // Completes network call or fails.
        .try_into()?; // Deserializes into Secret or fails.
    println!("set {} version {}", secret.name, secret.version);

    // Alternatively without the syntactic sugar above:
    let name = "secret-name";
    let secret: Secret = match client.set_secret(name, "secret-value", None, None).await {
        Ok(resp) => resp.try_into()?,
        Err(err) => {
            if let ErrorKind::HttpResponse { raw_response, .. } = err.kind() {
                eprintln!("failed to set secret {name}: {raw_response:?}");
            }
            std::process::exit(1)
        }
    };
    println!("set {} version {}", secret.name, secret.version);

    // More complex client method call.
    let mut ctx = Context::default();
    ctx.insert("example".to_string());

    let response = client
        .set_secret(
            "secret-name",
            "rotated-value",
            Some(SetSecretOptions {
                properties: Some(SecretProperties { enabled: false }),
                ..Default::default()
            }),
            Some(&ctx),
        )
        .await?;

    // Option 2: Implement async TryFrom<Response> for models, which customers can also do. Options are not mutually exclusive.
    // NOTE: async TryFrom<T> is still experimental but under consideration.
    let secret: Secret = response.try_into()?;
    println!("set {} version {}", secret.name, secret.version);

    // Concurrent client method calls with same options.
    let mut ctx = Context::default();
    ctx.insert("example".to_string());

    let options = SetSecretOptions {
        content_type: Some("text/plain".to_string()),
        ..Default::default()
    };

    let (_, _) = tokio::join!(
        client.set_secret("foo", "foo-value", Some(options.clone()), Some(&ctx)),
        client.set_secret("bar", "bar-value", Some(options.clone()), Some(&ctx)),
    );

    Ok(())
}
