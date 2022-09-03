#  Eframe app Website with wasm

## Local

### Native
```rust
cargo run --release
```
### Serving
```rust
cargo install trunk
```

Watches for changes and recomiles aswell as hosting a webserver locally
```rust
trunk serve --release
```

## Hosting
vikkasswe.github.io/website
Push `dist/` contents to `gh-pages` branch:
```rust
trunk build --release
```
