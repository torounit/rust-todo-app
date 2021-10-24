```bash
cd frontend
cargo install diesel_cli --no-default-features --features sqlite  
cargo install cargo-watch
cargo install trunk wasm-bindgen-cli
rustup target add wasm32-unknown-unknown 
trunk serve 

```