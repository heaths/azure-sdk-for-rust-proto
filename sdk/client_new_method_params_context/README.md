# Client constructor with parameter methods taking a context

This is a prototype of using a client "constructor" - an idiomatic `new` method - and passing method parameters for
required URL or query string parameters, as well as any required body, and a `Context` similar to Go. This `Context`
currently is only a property bag - a type map, more specifically, to provide unique and even protected values.

The `Context` could be used for other things passed through the call chain in the future. We require it, though callers
wrapping our APIs must also expose it to callers and should ideally add their own context e.g., for tracing.

## Examples

* [set_secret](examples/set_secret.rs)
