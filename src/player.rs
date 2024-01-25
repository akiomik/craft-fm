use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use web_sys::AudioContext;

use crate::sampler::Sampler;
use crate::note::Note;

#[wasm_bindgen]
pub struct Player {
    ctx: AudioContext,
    sampler: Sampler,
}

impl Drop for Player {
    fn drop(&mut self) {
        let _ = self.ctx.close();
    }
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Player, JsValue> {
        let ctx = AudioContext::new()?;
        let mut samples = HashMap::new();
        samples.insert(Note::A3, include_bytes!("../samples/a3.wav").as_slice().into());
        let sampler = Sampler::new(ctx.clone(), samples);

        Ok(Self { ctx, sampler })
    }

    pub async fn play(&self, note: Note) -> Result<(), JsValue> {
        let src = self.sampler.buffer_node(&note).await?;
        src.connect_with_audio_node(&self.ctx.destination())?;
        src.start()?;
        Ok(())
    }
}
