# zeiterfassung

See also https://github.com/pajoma/vscode-journal

## format
```bash
cargo fmt
```

## check warnings
```bash
cargo clippy -- -D warnings
cargo clippy --target wasm32-unknown-unknown -- -D warnings
```

## run local native
```bash
cargo run --release
```

## run local wasm
```bash
trunk serve --release
```
