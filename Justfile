
install-global-deps:
  cargo install basic-http-server wasm-bindgen-cli

serve: build-client
  basic-http-server web/

build-client: client-wasm-opt

client-wasm-opt: client-wasm-bindgen
  wasm-opt -Oz --output web/wasm/app_bg.wasm web/wasm/app_bg.wasm

client-wasm-bindgen: build-wasm
  wasm-bindgen --out-name app \
    --split-linked-modules \
    --out-dir web/wasm \
    --target web target/wasm32-unknown-unknown/wasm-release/app.wasm # Change to bundler target once vite web environment is configured

build-wasm:
  cargo build -p client --lib --profile wasm-release --target wasm32-unknown-unknown
