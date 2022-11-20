Easy and dumb project to test out yew.

MIT license.

### Install
```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
```

### Dev run
```bash
trunk serve
```

### Release build
```bash
trunk build --release
```