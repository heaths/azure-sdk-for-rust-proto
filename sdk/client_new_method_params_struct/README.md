# Client constructor with parameter methods

This is a prototype of using a client "constructor" - an idiomatic `new` method - and passing method parameters for
required URL or query string parameters, as well as any required body.

## Examples

* [set_secret](examples/set_secret.rs)

### Client options

Both the client and method take an optional client options bag via `Option<T>`. Options implement `Clone` and can be
cloned for those customers who want to build multiple clients from a single client options, or vary method calls.

```rust
use std::sync::Arc;
use azure_core::{ClientOptions, policies::CustomHeaderPolicy};
use azure_identity::{DefaultAzureCredential};
use azure_client_new_methods_params_struct::{SecretClient, SecretClientOptions};

let credential = Arc::new(DefaultAzureCredential::default());
let mut options = SecretClientOptions {
    api_version: "7.4".to_string(),
    options: ClientOptions {
        per_call_policies: vec![Arc::new(CustomHeaderPolicy::new("x-ms-custom-1", "foo"))],
        ..Default::default()
    },
};


// Simulate a call to clone shared client options, which is essentially frozen at this point.
let options1 = options.clone();
assert_eq!(options1.options.per_call_policies.len(), 1);

// NOTE: only cloned options1 since we reference it below and cannot move it like with options2 below.
let client1 = SecretClient::new("https://vault1.vault.azure.net", credential.clone(), Some(options1.clone()));

options.options.per_call_policies.push(Arc::new(CustomHeaderPolicy::new("x-ms-custom-2", "bar")));

// options1 (or even just options.clone() when used to create client1) remains frozen.
let options2 = options.clone();
assert_eq!(options1.options.per_call_policies.len(), 1);
assert_eq!(options2.options.per_call_policies.len(), 2);

// NOTE: we can - and should, for efficiency - move options2 unlike our simulation for options1 above.
let client2 = SecretClient::new("https://vault2.vault.azure.net", credential.clone(), Some(options2));
```
