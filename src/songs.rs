use wasm_bindgen::prelude::*;

// NOTE: A wrapper for Playable as workaround
//       because Box<dyn Playable> cannot be received by set_song
//       when #[wasm_bindgen(constructor)] is specified for Player::new
#[wasm_bindgen]
pub struct Song {
    #[allow(dead_code)]
    name: String,
    inner: Box<dyn Playable>,
}

impl Song {
    pub fn new(name: String, playable: Box<dyn Playable>) -> Self {
        Self {
            name,
            inner: playable,
        }
    }

    #[inline]
    pub fn play(&mut self) -> Result<(), JsValue> {
        self.inner.play()
    }

    #[inline]
    pub fn stop(&mut self) -> Result<(), JsValue> {
        self.inner.stop()
    }
}

pub trait Playable {
    fn play(&mut self) -> Result<(), JsValue>;

    fn stop(&mut self) -> Result<(), JsValue>;
}
