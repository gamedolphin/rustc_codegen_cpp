build:
    cargo build

[working-directory: 'example/simple']
build-example: build
    RUSTFLAGS="-Zcodegen-backend=../../target/debug/librustc_codegen_cpp.so" cargo build
