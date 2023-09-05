wasm-pack build --dev --target web
./run-web-server.py

# Using just wasm-bindgen
#cargo build --target=wasm32-unknown-unknown
#wasm-bindgen --target=nodejs --out-dir=tmp target/wasm32-unknown-unknown/debug/gpuwu.wasm
#./run-web-server.py
