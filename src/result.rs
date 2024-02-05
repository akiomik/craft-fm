use wasm_bindgen::JsValue;

pub type Result<A> = std::result::Result<A, JsValue>;
