#[derive(Debug, Default)]
pub struct DefaultAzureCredential {}

impl azure_core::TokenCredential for DefaultAzureCredential {}
