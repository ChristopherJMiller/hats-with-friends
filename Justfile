
install-global-deps:
  cargo install wasm-bindgen-cli@0.2.87

serve: build-client
  yarn workspace web dev

client:
  cargo run -p client

build-client: client-wasm-opt
  cp -r client/assets web/public/

client-wasm-opt: client-wasm-bindgen
  wasm-opt -Oz --output web/src/wasm/app_bg.wasm web/src/wasm/app_bg.wasm

client-wasm-bindgen: build-wasm
  wasm-bindgen --out-name app \
    --typescript \
    --split-linked-modules \
    --out-dir web/src/wasm \
    --target bundler target/wasm32-unknown-unknown/wasm-release/app.wasm 

build-wasm:
  cargo build -p client --lib --profile wasm-release --target wasm32-unknown-unknown

server:
  cargo run -p server
