use bstr::BString;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct Script {
    #[builder(setter(into))]
    content: BString,
}

impl Script {
    pub fn new(content: impl Into<BString>) -> Self {
        Self { content: content.into() }
    }

    pub fn content(&self) -> &[u8] {
        &self.content
    }
}
