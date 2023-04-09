#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {

        mod resources;
        mod systems;
        mod app;
        mod components;

        use wasm_bindgen::prelude::*;
        use shared::messages::Auth;
        use crate::resources::SESSION_AUTH_DATA;

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = console)]
            fn log(s: &str);
        }

        #[wasm_bindgen(js_name = setAuth)]
        pub fn set_auth(username: &str, password: &str) {
            if let Ok(mut data) = SESSION_AUTH_DATA.write() {
                log(&format!("Setting username to {}", username));
                *data = Some(Auth::new(username, password))
            } else {
                log("Failed to acquire session auth write lock");
            }
        }

        #[wasm_bindgen(start)]
        pub fn main() -> Result<(), JsValue> {
            app::run();

            Ok(())
        }
    }
}
