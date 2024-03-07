use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

// TODO: If we remove len() and is_empty() - which in the Azure/azure-sdk-for-rust repo are used only in tests - we could add a parent Arc<Context> and take those everywhere else to reduce memory on Context being potentially shared across threads.
#[derive(Clone, Debug)]
pub struct Context {
    type_map: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Context {
    fn new() -> Self {
        Self {
            type_map: HashMap::new(),
        }
    }

    #[cfg(feature = "context")]
    pub fn with_context(parent: &Context) -> Self {
        // Go does no copy context values, but they are practically read-only.
        // Cloning the HashMap isn't the cheapest solution but doe effectively handle overrides e.g., replacing an existing entity.
        // See https://cs.opensource.google/go/go/+/refs/tags/go1.22.0:src/context/context.go for Go's implementation.
        Self {
            type_map: parent.type_map.clone(),
        }
    }

    pub fn insert_or_replace<E>(&mut self, entity: E) -> Option<Arc<E>>
    where
        E: Send + Sync + 'static,
    {
        // We make sure that for every TypeId of E as key we ALWAYS retrieve an Option<Arc<E>>. That's why
        // the `unwrap` below is safe.
        self.type_map
            .insert(TypeId::of::<E>(), Arc::new(entity))
            .map(|displaced| displaced.downcast().expect("failed to unwrap downcast"))
    }

    pub fn insert<E>(&mut self, entity: E) -> &mut Self
    where
        E: Send + Sync + 'static,
    {
        self.type_map.insert(TypeId::of::<E>(), Arc::new(entity));

        self
    }

    pub fn remove<E>(&mut self) -> Option<Arc<E>>
    where
        E: Send + Sync + 'static,
    {
        self.type_map
            .remove(&TypeId::of::<E>())
            .map(|removed| removed.downcast().expect("failed to unwrap downcast"))
    }

    pub fn value<E>(&self) -> Option<&E>
    where
        E: Send + Sync + 'static,
    {
        self.type_map
            .get(&TypeId::of::<E>())
            .and_then(|item| item.downcast_ref())
    }

    pub fn len(&self) -> usize {
        self.type_map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.type_map.is_empty()
    }
}

impl Default for Context {
    fn default() -> Self {
        Context::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct A(String);
    impl A {
        fn new(value: impl Into<String>) -> Self {
            Self(value.into())
        }
    }

    struct B(i32);
    impl B {
        fn new(value: i32) -> Self {
            Self(value)
        }
    }

    #[test]
    fn with_parent() {
        let mut parent = Context::new();
        parent.insert(A::new("foo"));

        assert_eq!(parent.len(), 1);

        let mut sut = Context::with_context(&parent);
        sut.insert_or_replace(A::new("bar"));
        sut.insert(B::new(1));

        assert_eq!(sut.len(), 2);

        parent.insert_or_replace(A::new("baz"));

        assert_eq!(parent.len(), 1);
        assert_eq!(sut.len(), 2);

        assert_eq!(parent.value::<A>().expect("expected value").0, "baz");
        assert_eq!(sut.value::<A>().expect("expected value").0, "bar");
        assert_eq!(sut.value::<B>().expect("expected value").0, 1);
    }
}
