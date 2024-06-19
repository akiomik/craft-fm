use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, Worker};

use crate::result::Result;

#[wasm_bindgen]
pub struct WebWorker {
    closure: Option<Closure<dyn FnMut(MessageEvent)>>,
    handle: Worker,
}

impl WebWorker {
    pub fn new(name: &str) -> Result<WebWorker> {
        let handle = Worker::new(name)?;

        Ok(WebWorker {
            closure: None,
            handle,
        })
    }

    pub fn set_onmessage<F>(&mut self, f: F)
    where
        F: 'static + FnMut(MessageEvent),
    {
        let closure = Closure::new(f);
        self.handle
            .set_onmessage(Some(closure.as_ref().unchecked_ref()));
        self.closure = Some(closure);
    }

    pub fn post_message(&self, s: &str) -> Result<()> {
        self.handle.post_message(&JsValue::from_str(s))?;
        Ok(())
    }
}

impl Drop for WebWorker {
    fn drop(&mut self) {
        self.handle.terminate();
    }
}
