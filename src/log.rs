use wasm_bindgen::JsValue;
use web_sys::console;

#[allow(dead_code)]
pub fn log(s: String) {
    console::log_1(&JsValue::from(s));
}
