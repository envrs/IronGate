use bstr::BString;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Default, TypedBuilder)]
pub struct Environment {
    #[builder(default, setter(into))]
    variables: HashMap<BString, BString>,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: impl Into<BString>, value: impl Into<BString>) {
        self.variables.insert(key.into(), value.into());
    }

    pub fn variables(&self) -> &HashMap<BString, BString> {
        &self.variables
    }
}
