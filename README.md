# IronGate Encore

High-performance data encoding, hashing, and conversion engine with first-class WASM support.

## Project Structure

- `crates/irongate-encore`: Core Rust library containing high-performance implementations for:
    - **Encoding**: URL, Base64, Hex, Base32Hex, HTML entities.
    - **Hashing**: MD5, SHA1, SHA2 (224/256/384/512).
- `crates/irongate-shell`: Cross-platform asynchronous shell script execution without temporary files.
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

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
