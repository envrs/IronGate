use rusqlite::{Connection, Result};
use crate::cache::RegexCache;
use crate::operator::RegexpOperator;

pub struct SQLiteRegexWrapper;

impl SQLiteRegexWrapper {
    pub fn register(conn: &Connection) -> Result<()> {
        let cache = RegexCache::new();
        let operator = RegexpOperator::new(cache);

        conn.create_scalar_function(
            "regexp",
            2,
            rusqlite::functions::FunctionFlags::SQLITE_DETERMINISTIC,
            move |ctx| {
                let pattern: String = ctx.get(0)?;
                let text: String = ctx.get(1)?;

                operator.is_match(&pattern, &text)
                    .map_err(|e| rusqlite::Error::UserFunctionError(Box::new(e)))
            },
        )
    }
}
