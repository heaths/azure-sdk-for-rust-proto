# Azure/azure-sdk-for-net prototypes

[![ci](https://github.com/heaths/azure-sdk-for-rust-proto/actions/workflows/ci.yml/badge.svg?event=push)](https://github.com/heaths/azure-sdk-for-rust-proto/actions/workflows/ci.yml)

This repo is to basically prototype different ideas for [Azure/azure-sdk-for-rust] to create our [guidelines].
Nothing herein should be interpreted as a matter of policy.

## Examples

Find all examples under the [`sdk/`](sdk/) directory. The following, in no particular order, are what we are focusing on currently:

* [client_new_method_params]

  Construct clients with `new(...)` and call methods with required parameters and optional call parameters including policy customizations.

  Clients are immutable so long as the client options are cloned or copied.

  **Pros**

  * Similar feel to most other languages.
  * Can share options across multiple, distinct client method calls.

  **Cons**

  * Either callers or the callee must allocate, or we use a single lazy allocation of the `Default` implementation.
  * Callers may set ambiguous or conflicting options.

* [client_new_method_params_context]

  Similar to [client_new_method_params] but requires a `&Context` parameter declared first in the list of parameters.
  This is similar to Go and can be used for OpenTelemetry tracing, but is not used for canceling a `Future` - part of
  Rust's async framework. To cancel a `Future` you merely need to `drop()` it. Currently, this `Context` might only be
  a property bag.

  Clients are immutable so long as the client options are cloned or copied.

  Similar pros and cons to [client_new_method_params], but:

  **Pros**

  * For those callers who want to add context or who may expose context to their own callers when wrapping our APIs,
    this may be more obvious that something should be passed.

  **Cons**

  * Most callers will end up passing `Context::default` (by value or reference) for the first param, which is not common.
    Even passing as an `Option<Context>` means most customers will pass `None`, and having to pass multiple `None`s is uncommon.
    Comparing to other languages besides Go where `context.Context` is already idiomatic, few callers pass optional context.

* [client_builder_method_builder]

  Builders are fairly common in Rust, used in the likes of the AWS SDK (only for method calls), Bevy (ECS game engine),
  and some others. I wouldn't go so far as saying they are idiomatic, though: many projects even for cloud-related crates
  don't use them. But they do help validate or constrain inputs and can even be used with the
  [typestate builder pattern](https://gist.github.com/heaths/1eb608df947de5d5b47da0ee6a5a5c6d) to enforce
  mutually-exclusive settings or required groups of settings in ways that setting public fields or calling field setters
  cannot do.

  This prototype uses builders for both client - and, more notably, client options - as well as service method calls.

  Client builders as prototyped are mutable, but clients created from `build(&self) -> Client` are immutable.
  Client method builders are also mutable until `send(&self) -> BoxFuture<Result<Response>>` is called.

  **Pros**

  * If using builders at all, this is consistently using them for client creation and client method calls.
  * Can lead callers down the path of success using the [typestate builder pattern] when necessary.
  * Can validate options beyond type safety since setters are used.
  * Consistent with some other larger frameworks like `bevy`.

  **Cons**

  * Method builders do not support sharing common options across multiple, distinct client method calls.

* [client_new_method_builder]

  A combination of [client_new_method_params] to construct clients using `new(...)` and [client_builder_method_builder]
  using a builder for methods to optionally configure per-call settings.

  This is similar to how the [AWS SDK](https://awslabs.github.io/aws-sdk-rust/) is defined.

  Clients are immutable so long as the client options are cloned or copied.
  Client method builds are mutable until `send(&self) -> BoxFuture<Result<Response>>` is called.

  Similar pros and cons to [client_builder_method_builder], but:

  **Pros**

  * Consistent with the AWS SDKs, so familiar call patterns may ease migration to Azure.

  **Cons**

  * Inconsistent - perhaps even jarring - to use a client "constructor" but builders for client methods.

[client_builder_method_builder]: sdk/client_builder_method_builder/examples/set_secret_client_builder.rs
[client_new_method_builder]: sdk/client_new_method_builder/examples/set_secret_method_builder.rs
[client_new_method_params]: sdk/client_new_method_params/examples/set_secret.rs
[client_new_method_params_context]: sdk/client_new_method_params_context/examples/set_secret_with_context.rs

## Conventions

* Builder fields should be private.
* Builder setters should be declared as `with_field_name(&mut self, value: impl Into<FieldType>) -> &mut Self`.
* Client fields should be private.
* ClientOptions should be public.
* Model fields should be public.
* Parameter types that do not require concrete types should use `impl Into<ParamType>` for owned values or `impl AsRef<ParamType>` if not owned.
* Private field getters should be declared as `field_name(&self) -> &field_type`.
* Private field setters should be declared as `set_field_name(&mut self, value: impl Into<FieldType>)`.
* Type construction should use `new(...)` or `with_more_details(...)`. Consider implementing `Default` as appropriate.

See <https://rust-lang.github.io/api-guidelines/naming.html> for additional naming guidelines.

## License

Licensed under the [MIT](LICENSE.txt) license.

[Azure/azure-sdk-for-rust]: https://github.com/Azure/azure-sdk-for-rust
[guidelines]: https://azure.github.io/azure-sdk/general_introduction.html
[typestate builder pattern]: https://gist.github.com/heaths/1eb608df947de5d5b47da0ee6a5a5c6d
