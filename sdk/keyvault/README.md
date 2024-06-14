# Client constructor with parameter methods but options builder

Client construction and client method invocation uses parameters normally,
but options - which can be used to construct other clients or invoke subsequent methods,
are constructed using a builder pattern.

## Examples

* [set_secret_options_builder](examples/set_secret_options_builder.rs)

### Client options

Consider building an options bag struct:

```rust no_test
let options = SetSecretOptions {
    enabled: true,
    client_options: Some(azure_core::ClientMethodOptions {
        retry: Some(azure_core::Retry::exponential()),
        context: Some(ctx),
        ..Default::default()
    }),
    ..Default::default()
};
let response = client.set_secret("name", "value", Some(options)).await?;
```

Or using an options builder:

```rust no_test
let options = SetSecretOptions::builder()
    .with_enabled(true)
    .with_retry(azure_core::Retry::exponential())
    .with_context(&ctx)
    .build();
let response = client.set_secret("name", "value", Some(options)).await?;
```

We can also add methods to `SetSecretOptions` via `azure_core::ClientMethodOptions` without callers having to express
`..Default::default()` for every options struct even if they specified all available at the time the code was written.
