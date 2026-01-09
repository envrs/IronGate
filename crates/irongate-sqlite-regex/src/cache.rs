use regex::Regex;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct RegexCache {
    inner: Arc<Mutex<HashMap<String, Regex>>>,
}

impl RegexCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_or_insert(&self, pattern: &str) -> Result<Regex, regex::Error> {
        let mut cache = self.inner.lock().unwrap();
        if let Some(re) = cache.get(pattern) {
            Ok(re.clone())
        } else {
            let re = Regex::new(pattern)?;
            cache.insert(pattern.to_string(), re.clone());
            Ok(re)
        }
    }
}
