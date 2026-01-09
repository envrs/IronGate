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

# Build the Actix SSE library
build-actix-sse:
    cargo build -p irongate-actix-sse

# Build the TLS impersonation libraries
build-tls-imperson:
    cargo build -p tls-imperson
    cargo build -p tls-imperson-openssl

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

# Run Actix SSE tests
test-actix-sse:
    cargo test -p irongate-actix-sse

# Run TLS impersonation tests
test-tls-imperson:
    cargo test -p tls-imperson
    cargo test -p tls-imperson-openssl

# Build WASM bindings using wasm-pack
build-wasm:
    cd crates/encore && wasm-pack build --target nodejs --out-dir ../../wasm-package

# Build everything
build-all: build-core build-shell build-sqlite-regex build-process-alive build-actix-sse build-tls-imperson build-wasm

# Run web integration tests
test-web: build-wasm
    cd tests/web && yarn test

# Run all tests (core + shell + sqlite-regex + process-alive + actix-sse + tls-imperson + web)
test-all: test-core test-shell test-sqlite-regex test-process-alive test-actix-sse test-tls-imperson test-web

# Clean build artifacts
clean:
    cargo clean
    rm -rf wasm-package
