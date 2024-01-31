use wasm_bindgen::JsValue;
use web_sys::console;

#[allow(dead_code)]
pub fn log<S: Into<String>>(s: S) {
    console::log_1(&JsValue::from(s.into()));
}
