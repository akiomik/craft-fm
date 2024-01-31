use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use web_sys::AudioContext;

use crate::{
    note::Note,
    sampler::MelodicSampler,
    sequencer::{Resolution, Sequencer},
};

use super::{Playable, Song};

#[wasm_bindgen]
pub struct Metronome {
    ctx: AudioContext,
    sampler: MelodicSampler,
    sequencer: Sequencer,
}

#[wasm_bindgen]
impl Metronome {
    #[wasm_bindgen(constructor)]
    pub async fn new(ctx: AudioContext, bpm: usize) -> Result<Metronome, JsValue> {
        let mut samples = HashMap::new();
        samples.insert(
            Note::A2,
            include_bytes!("../../samples/a2.m4a").as_slice().into(),
        );
        samples.insert(
            Note::A3,
            include_bytes!("../../samples/a3.m4a").as_slice().into(),
        );
        let sampler = MelodicSampler::new(ctx.clone(), samples).await?;
        let sequencer = Sequencer::new(ctx.clone(), bpm, 1, Resolution::Quarter, 100)?;

        Ok(Self {
            ctx,
            sampler,
            sequencer,
        })
    }

    // NOTE: Can't use `impl Into<Song>` and `impl From<Metronome>` on js
    #[wasm_bindgen]
    pub fn into_song(self) -> Song {
        Song::new("metronome".into(), Box::new(self))
    }
}

impl Playable for Metronome {
    fn play(&mut self) -> Result<(), JsValue> {
        let ctx = self.ctx.clone();
        let sampler = self.sampler.clone();

        self.sequencer.start(move |time, step, _page| {
            let src = if step == 0 {
                sampler.buffer_node(&Note::C4)?
            } else {
                sampler.buffer_node(&Note::C3)?
            };

            src.connect_with_audio_node(&ctx.destination())?;
            src.start_with_when(time)?;

            Ok(())
        })
    }

    fn stop(&mut self) -> Result<(), JsValue> {
        self.sequencer.stop()
    }
}
