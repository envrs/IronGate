use regex::Regex;
use rusqlite::{Connection, Result};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

/// Registers the `regexp` function to a SQLite connection.
///
/// This allows using the `REGEXP` operator in SQL queries:
/// `SELECT * FROM table WHERE column REGEXP 'pattern'`
pub fn register_regexp_function(conn: &Connection) -> Result<()> {
    // Shared regex cache to avoid recompiling same patterns
    let cache: Arc<Mutex<HashMap<String, Regex>>> = Arc::new(Mutex::new(HashMap::new()));

    conn.create_scalar_function(
        "regexp",
        2,
        rusqlite::functions::FunctionFlags::SQLITE_DETERMINISTIC,
        move |ctx| {
            let pattern_str: String = ctx.get(0)?;
            let text: String = ctx.get(1)?;

            let mut cache = cache.lock().unwrap();

            let regex = if let Some(re) = cache.get(&pattern_str) {
                re
            } else {
                match Regex::new(&pattern_str) {
                    Ok(re) => {
                        cache.insert(pattern_str.clone(), re);
                        cache.get(&pattern_str).unwrap()
                    }
                    Err(e) => return Err(rusqlite::Error::UserFunctionError(Box::new(e))),
                }
            };

            Ok(regex.is_match(&text))
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_regexp_operator() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        register_regexp_function(&conn)?;

        conn.execute("CREATE TABLE test (name TEXT)", [])?;
        conn.execute("INSERT INTO test (name) VALUES ('Alice')", [])?;
        conn.execute("INSERT INTO test (name) VALUES ('Bob')", [])?;
        conn.execute("INSERT INTO test (name) VALUES ('Charlie')", [])?;

        // Simple match
        let mut stmt = conn.prepare("SELECT count(*) FROM test WHERE name REGEXP '^A'")?;
        let count: i32 = stmt.query_row([], |r| r.get(0))?;
        assert_eq!(count, 1);

        // Case sensitive match (regex is case sensitive by default)
        let mut stmt = conn.prepare("SELECT count(*) FROM test WHERE name REGEXP 'alice'")?;
        let count: i32 = stmt.query_row([], |r| r.get(0))?;
        assert_eq!(count, 0);

        // Advanced regex
        let mut stmt = conn.prepare("SELECT count(*) FROM test WHERE name REGEXP 'rli'")?;
        let count: i32 = stmt.query_row([], |r| r.get(0))?;
        assert_eq!(count, 1); // Charlie

        Ok(())
    }

    #[test]
    fn test_invalid_regex() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        register_regexp_function(&conn)?;

        // Test that an invalid regex causes an error
        let res = conn.execute("SELECT * FROM (SELECT 'test' as name) WHERE name REGEXP '['", []);
        assert!(res.is_err());

        Ok(())
    }
}
