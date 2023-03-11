
install-global-deps:
  cargo install basic-http-server wasm-bindgen-cli

serve: build-client
  basic-http-server web/

build-client: client-wasm-opt

client-wasm-opt: client-wasm-bindgen
  wasm-opt -Oz --output web/wasm/client_bg.wasm web/wasm/client_bg.wasm

client-wasm-bindgen: build-wasm
  wasm-bindgen --out-name client \
    --out-dir web/wasm \
    --target web target/wasm32-unknown-unknown/wasm-release/hats-with-friends.wasm

build-wasm:
  cargo build --profile wasm-release --target wasm32-unknown-unknown
