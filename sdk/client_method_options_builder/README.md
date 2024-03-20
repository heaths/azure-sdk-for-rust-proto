# Client constructor with parameter methods but options builder

This prototype is similar to [client_new_method_params] but uses a builder just for client and client method options.
This still allows for sharing options, but has the discoverability, succinctness, and expansion potential of using
builders like with [client_builder_method_builder].

## Examples

* [set_secret_options_builder](examples/set_secret_options_builder.rs)

### Client options

Consider building an options bag struct for [client_new_method_params]:

```rust
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

```rust
let options = SetSecretOptions::builder()
    .with_enabled(true)
    .with_retry(azure_core::Retry::exponential())
    .with_context(&ctx)
    .build();
let response = client.set_secret("name", "value", Some(options)).await?;
```

We can also add methods to `SetSecretOptions` via `azure_core::ClientMethodOptions` without callers having to express
`..Default::default()` for every options struct even if they specified all available at the time the code was written.

[client_builder_method_builder]: sdk/client_builder_method_builder/examples/set_secret_client_builder.rs
[client_new_method_params]: sdk/client_new_method_params/examples/set_secret_params.rs
