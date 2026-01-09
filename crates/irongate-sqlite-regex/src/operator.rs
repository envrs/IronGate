use crate::cache::RegexCache;

pub struct RegexpOperator {
    cache: RegexCache,
}

impl RegexpOperator {
    pub fn new(cache: RegexCache) -> Self {
        Self { cache }
    }

    pub fn is_match(&self, pattern: &str, text: &str) -> Result<bool, regex::Error> {
        let re = self.cache.get_or_insert(pattern)?;
        Ok(re.is_match(text))
    }
}
