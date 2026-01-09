# Default recipe
default: test-all

# Build the core irongate-encore crate
build-core:
    cargo build -p irongate-encore

# Run core library tests
test-core:
    cargo test -p irongate-encore

# Build WASM bindings using wasm-pack
build-wasm:
    cd crates/encore && wasm-pack build --target nodejs --out-dir ../../wasm-package

# Build everything
build-all: build-core build-wasm

# Run web integration tests
test-web: build-wasm
    cd tests/web && yarn test

# Run all tests (core + web)
test-all: test-core test-web

# Clean build artifacts
clean:
    cargo clean
    rm -rf wasm-package
