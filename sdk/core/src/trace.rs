#[derive(Clone, Debug)]
pub struct Span(&'static str);

impl From<&'static str> for Span {
    fn from(name: &'static str) -> Self {
        Self(name)
    }
}
