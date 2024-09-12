use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::console;

mod oauth_client;
mod token;
mod http;
mod utils;
mod error;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Wasm'de kullanıma açılacak modülleri dışa aktaralım
#[wasm_bindgen]
pub fn authorize() -> String {
    oauth_client::authorize()
}

#[wasm_bindgen]
pub fn exchange_code(code: String) -> js_sys::Promise {
    future_to_promise(oauth_client::exchange_code(code))
}
