# rust_wasm_sample

Super quick and dirty demo for investigating Rust + WASM.  Build with:
```cmd
cargo install wasm-pack
wasm-pack build --target no-modules
```
Then open [index.html](index.html), preferably in FireFox as Chrome blocks WASM over `file://` via CORS.
