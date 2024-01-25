use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, Worker};

#[wasm_bindgen]
pub struct WebWorker {
    closure: Option<Closure<dyn FnMut(MessageEvent)>>,
    handle: Worker,
}

impl WebWorker {
    pub fn new(name: &str) -> Result<WebWorker, JsValue> {
        let handle = Worker::new(name)?;

        Ok(WebWorker {
            closure: None,
            handle,
        })
    }

    pub fn set_onmessage<F: 'static>(&mut self, f: F)
    where
        F: FnMut(MessageEvent),
    {
        let closure = Closure::new(f);
        self.handle
            .set_onmessage(Some(closure.as_ref().unchecked_ref()));
        self.closure = Some(closure);
    }

    pub fn post_message(&self, s: &str) -> Result<(), JsValue> {
        self.handle.post_message(&JsValue::from_str(s))?;
        Ok(())
    }
}
