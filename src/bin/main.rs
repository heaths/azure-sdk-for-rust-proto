// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", azure_sdk_for_rust_proto::say_hello(None));
    Ok(())
}
