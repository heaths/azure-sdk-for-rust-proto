use azure_client_new_methods_params::{
    Secret, SecretClient, SecretClientOptions, SetSecretOptions,
};
use azure_core::Context;
use azure_identity::DefaultAzureCredential;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = env::var("AZURE_KEYVAULT_URL")?;

    let credential = Arc::new(DefaultAzureCredential::default());
    let options = SecretClientOptions {
        api_version: "7.4".to_string(),
        ..Default::default()
    };
    let client = SecretClient::new(endpoint, credential, Some(options))?;

    let mut ctx = Context::default();
    ctx.insert("example".to_string());

    let response = client
        .set_secret(
            "secret-name",
            "secret-value",
            // Caller would otherwise pass `None` with no options.
            Some(SetSecretOptions {
                context: Some(ctx),
                ..Default::default()
            }),
        )
        .await?;

    let secret: Secret = response.json().await?;
    println!("set {} version {}", secret.name, secret.version);

    // Option 2: Implement async TryFrom<Response> for models, which customers can also do. Options are not mutually exclusive.
    // Note: async TryFrom<T> is still experimental but under consideration.

    Ok(())
}
