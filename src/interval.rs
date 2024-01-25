use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn clearInterval(handle: f64);
}

#[wasm_bindgen]
pub struct Interval {
    _closure: Closure<dyn FnMut()>,
    handle: f64,
}

impl Interval {
    pub fn new<F: 'static>(f: F, millis: u32) -> Interval
    where
        F: FnMut(),
    {
        let _closure = Closure::new(f);
        let handle = setInterval(&_closure, millis);

        Interval { _closure, handle }
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
        clearInterval(self.handle);
    }
}
