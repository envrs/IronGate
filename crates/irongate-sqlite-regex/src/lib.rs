pub mod cache;
pub mod operator;
pub mod wrapper;

pub use cache::RegexCache;
pub use operator::RegexpOperator;
pub use wrapper::SQLiteRegexWrapper;

use rusqlite::{Connection, Result};

/// Registers the `regexp` function to a SQLite connection.
///
/// This allows using the `REGEXP` operator in SQL queries:
/// `SELECT * FROM table WHERE column REGEXP 'pattern'`
pub fn register_regexp_function(conn: &Connection) -> Result<()> {
    SQLiteRegexWrapper::register(conn)
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
