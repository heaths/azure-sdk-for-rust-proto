use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

#[derive(Clone, Debug)]
pub struct Context {
    type_map: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            type_map: HashMap::new(),
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Context::new()
    }
}
