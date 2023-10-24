# zeiterfassung

[Web App](https://dakup.github.io/zeiterfassung)

See also
- https://github.com/pajoma/vscode-journal
- https://github.com/jsynowiec/vscode-insertdatestring

## run local native
```bash
cargo run --release
```

## run local wasm
```bash
trunk serve --release
```

## format code
```bash
cargo fmt
```

## check warnings
```bash
cargo clippy -- -D warnings
cargo clippy --target wasm32-unknown-unknown -- -D warnings
```
