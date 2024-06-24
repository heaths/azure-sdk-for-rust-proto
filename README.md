# Azure/azure-sdk-for-net prototypes

[![ci](https://github.com/heaths/azure-sdk-for-rust-proto/actions/workflows/ci.yml/badge.svg?event=push)](https://github.com/heaths/azure-sdk-for-rust-proto/actions/workflows/ci.yml)

This repo is a prototype for [Azure/azure-sdk-for-rust] to help create our [guidelines].
We have chosen the options builder pattern after a lot of consideration and research, but will make adjustments based on
feedback as we progress.

## Example

An example of the options builder pattern we have chosen:

```rust
let endpoint = env::var("AZURE_KEYVAULT_URL")?;

let credential = Arc::new(DefaultAzureCredential::default());
let options = SecretClientOptions::builder()
    .with_api_version("7.4")
    .with_retry(RetryOptions::exponential(ExponentialRetryOptions::default()))
    .build();
let client = SecretClient::new(endpoint, credential, Some(options))?;

// Simple client method call.
let response = client
    .set_secret("secret-name", "secret-value", None)
    .await?;

let secret: Secret = response.json().await?;
println!("set {} version {}", secret.name, secret.version);
```

## Conventions

* Builder fields should be private.
* Builder setters should be declared as `with_field_name(&mut self, value: impl Into<FieldType>) -> &mut Self`.
* Client fields should be private.
* ClientOptions should be public.
* Model fields should be public.
* Parameter types that do not require concrete types should use `impl Into<ParamType>` for owned values
  or `impl AsRef<ParamType>` if not owned.
* Private field getters should be declared as `field_name(&self) -> &field_type`.
* Private field setters should be declared as `set_field_name(&mut self, value: impl Into<FieldType>)`.
* Type construction should use `new(...)` or `with_more_details(...)`. Consider implementing `Default` as appropriate.

Read our [guidelines] for more information,
and <https://rust-lang.github.io/api-guidelines/naming.html> for additional naming guidelines.

## License

Licensed under the [MIT](LICENSE.txt) license.

[Azure/azure-sdk-for-rust]: https://github.com/Azure/azure-sdk-for-rust
[guidelines]: https://azure.github.io/azure-sdk/rust_introduction.html
[typestate builder pattern]: https://gist.github.com/heaths/1eb608df947de5d5b47da0ee6a5a5c6d
