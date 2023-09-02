wasm-pack build --dev --debug --target web
python3 -m http.server

# Using just wasm-bindgen
#cargo build --target=wasm32-unknown-unknown
#wasm-bindgen --target=nodejs --out-dir=tmp target/wasm32-unknown-unknown/debug/gpuwu.wasm
#python3 -m http.server
