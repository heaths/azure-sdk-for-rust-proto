# Azure/azure-sdk-for-net prototypes

[![ci](https://github.com/heaths/azure-sdk-for-rust-proto/actions/workflows/ci.yml/badge.svg?event=push)](https://github.com/heaths/azure-sdk-for-rust-proto/actions/workflows/ci.yml)

This repo is to basically prototype different ideas for [Azure/azure-sdk-for-rust] to create our [guidelines].
Nothing herein should be interpreted as a matter of policy.

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
