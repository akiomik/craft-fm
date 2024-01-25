use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn clearInterval(handle: f64);
}

#[wasm_bindgen]
pub struct Interval {
    #[allow(dead_code)]
    closure: Closure<dyn FnMut()>,
    handle: f64,
}

impl Interval {
    pub fn new<F: 'static>(f: F, millis: u32) -> Interval
    where
        F: FnMut(),
    {
        let closure = Closure::new(f);
        let handle = setInterval(&closure, millis);
        Interval { closure, handle }
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
        clearInterval(self.handle);
    }
}
