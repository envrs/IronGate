# IronGate Encore

High-performance data encoding, hashing, and conversion engine with first-class WASM support.

## Project Structure

- `crates/irongate-encore`: Core Rust library containing high-performance implementations for:
    - **Encoding**: URL, Base64, Hex, Base32Hex, HTML entities.
    - **Hashing**: MD5, SHA1, SHA2 (224/256/384/512).
- `crates/irongate-shell`: Cross-platform asynchronous shell script execution without temporary files.
- `crates/irongate-sqlite-regex`: A regular expression SQLite extension that enables the `REGEXP` operator.
- `crates/irongate-process-alive`: Efficient cross-platform process liveness checking.
- `crates/encore`: WASM bindings that wrap the core library for use in JavaScript/TypeScript environments.
- `tests/web`: Integration test suite for the WASM package.

## Development

This project uses `just` for task automation.

### Prerequisites

- [Rust](https://www.rust-lang.org/) (stable)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Just](https://github.com/casey/just)

### Common Tasks

```bash
# Run all tests (Core + Shell + WASM)
just test-all

# Build core library
just build-core

# Build shell library
just build-shell

# Build SQLite regex library
just build-sqlite-regex

# Build process alive library
just build-process-alive

# Build WASM bindings
just build-wasm

# Run web integration tests
just test-web
```

## Usage (WASM)

The WASM package is designed to be highly efficient and easy to use in Node.js or browser environments.

```javascript
import { Md5Hash, UrlEncode } from "./wasm-package/encore.js";

// Hashing
const hasher = new Md5Hash();
const hash = hasher.apply(new TextEncoder().encode("irongate"));

// URL Encoding
const encoder = new UrlEncode({ non_ascii: true, charset: "e" });
const encoded = encoder.apply(new TextEncoder().encode("irongate @éé"));
```

## Usage (Rust Shell Executor)

```rust
use irongate_shell::ShellExecutor;

#[tokio::main]
async fn main() {
    let executor = ShellExecutor::builder().build();
    let output = executor.execute("echo 'Hello from IronGate Shell'").await.unwrap();
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
```

## Usage (SQLite Regex Extension)

```rust
use rusqlite::Connection;
use irongate_sqlite_regex::register_regexp_function;

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?;
    register_regexp_function(&conn)?;

    let mut stmt = conn.prepare("SELECT 'Alice' REGEXP '^A'")?;
    let found: bool = stmt.query_row([], |r| r.get(0))?;
    assert!(found);
    Ok(())
}
```

## Usage (Process Alive)

```rust
use irongate_process_alive::{is_alive, ProcessExt};

fn main() {
    let pid = 1234;
    
    // Using function
    let status = is_alive(pid);
    println!("Process {} status: {}", pid, status);
    
    // Using trait extension
    if pid.status().is_alive() {
        println!("It's alive!");
    }
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
