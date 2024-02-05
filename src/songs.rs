use wasm_bindgen::prelude::*;

use crate::result::Result;

pub mod forest;
pub mod metronome;
pub mod toy808;

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
    pub fn new<S: Into<String>>(name: S, playable: Box<dyn Playable>) -> Self {
        Self {
            name: name.into(),
            inner: playable,
        }
    }

    #[inline]
    pub fn tick(&mut self) -> Result<()> {
        self.inner.tick()
    }
}

pub trait Playable {
    fn tick(&mut self) -> Result<()>;
}
