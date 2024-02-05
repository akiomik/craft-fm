use wasm_bindgen::prelude::*;

use crate::result::Result;

#[wasm_bindgen]
pub struct Interval {
    _closure: Closure<dyn FnMut()>,
    handle: i32,
}

impl Interval {
    pub fn new<F: 'static>(f: F, millis: u32) -> Result<Interval>
    where
        F: FnMut(),
    {
        let window = web_sys::window().unwrap();
        let _closure = Closure::new(f);
        let handle = window.set_interval_with_callback_and_timeout_and_arguments_0(
            _closure.as_ref().unchecked_ref(),
            millis as i32,
        )?;

        Ok(Interval { _closure, handle })
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
        let window = web_sys::window().unwrap();
        window.clear_interval_with_handle(self.handle);
    }
}
