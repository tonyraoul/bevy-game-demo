# Bevy Demo - Spinning Cube

A simple demo showing a spinning cube using the Bevy game engine, with support for both native macOS and web platforms.

## Prerequisites

1. Rust and Cargo (install via [rustup](https://rustup.rs/))
2. For web builds:
   ```bash
   rustup target add wasm32-unknown-unknown
   cargo install wasm-bindgen-cli
   ```

## Building and Running

### Native (macOS)

```bash
# Run in debug mode
cargo run

# Run in release mode
cargo run --release
```

### Web

```bash
# Build for web
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./target/wasm --target web ./target/wasm32-unknown-unknown/release/bevy-demo.wasm

# Serve the content (using Python's built-in server as an example)
python3 -m http.server
```

Then open your browser and navigate to `http://localhost:8000`

## Controls

The cube will automatically spin on its Y and X axes. The camera is positioned to view the cube from a slight elevation. 