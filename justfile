# Default recipe
default: test-all

# Build the core irongate-encore crate
build-core:
    cargo build -p irongate-encore

# Build the shell execution library
build-shell:
    cargo build -p irongate_shell

# Build the SQLite regex extension
build-sqlite-regex:
    cargo build -p irongate-sqlite-regex

# Build the process alive library
build-process-alive:
    cargo build -p irongate-process-alive

# Run core library tests
test-core:
    cargo test -p irongate-encore

# Run shell library tests
test-shell:
    cargo test -p irongate_shell

# Run SQLite regex tests
test-sqlite-regex:
    cargo test -p irongate-sqlite-regex

# Run process alive tests
test-process-alive:
    cargo test -p irongate-process-alive

# Build WASM bindings using wasm-pack
build-wasm:
    cd crates/encore && wasm-pack build --target nodejs --out-dir ../../wasm-package

# Build everything
build-all: build-core build-shell build-sqlite-regex build-process-alive build-wasm

# Run web integration tests
test-web: build-wasm
    cd tests/web && yarn test

# Run all tests (core + shell + sqlite-regex + process-alive + web)
test-all: test-core test-shell test-sqlite-regex test-process-alive test-web

# Clean build artifacts
clean:
    cargo clean
    rm -rf wasm-package
