use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use web_sys::AudioContext;

use crate::sampler::Sampler;
use crate::note::Note;
use crate::sequencer::{Resolution, Sequencer};

#[wasm_bindgen]
pub struct Player {
    ctx: AudioContext,
    sampler: Sampler,
    sequencer: Sequencer,
}

impl Drop for Player {
    fn drop(&mut self) {
        let _ = self.ctx.close();
    }
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(constructor)]
    pub async fn new() -> Result<Player, JsValue> {
        let ctx = AudioContext::new()?;
        let mut samples = HashMap::new();
        samples.insert(Note::A3, include_bytes!("../samples/a3.wav").as_slice().into());
        let sampler = Sampler::new(ctx.clone(), samples).await?;
        let sequencer = Sequencer::new(ctx.clone(), 120, Resolution::Eighth, 250);

        Ok(Self { ctx, sampler, sequencer })
    }

    pub fn play(&mut self, note: Note) -> Result<(), JsValue> {
        self.sequencer.stop();

        let ctx = self.ctx.clone();
        let sampler = self.sampler.clone();

        self.sequencer.start(move |time, _| {
            let src = sampler.buffer_node(&note)?;
            src.connect_with_audio_node(&ctx.destination())?;
            src.start_with_when(time)?;
            Ok(())
        })?;

        Ok(())
    }
}
