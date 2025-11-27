use wasm_bindgen::prelude::*;
use web_sys::AudioContext;

use crate::{
    result::Result,
    sampler::MelodicSampler,
    sequencer::{Resolution, Sequencer},
    theory::Note,
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
    pub fn new(ctx: AudioContext, bpm: f32) -> Metronome {
        let sequencer = Sequencer::new(bpm, 1, Resolution::Quarter, ctx.current_time(), 100);

        Self {
            ctx: ctx.clone(),
            sampler: MelodicSampler::new(ctx),
            sequencer,
        }
    }

    #[wasm_bindgen]
    pub async fn init(&mut self) -> Result<()> {
        self.sampler
            .insert(Note::A2, include_bytes!("../../samples/a2.m4a"))
            .await?;
        self.sampler
            .insert(Note::A3, include_bytes!("../../samples/a3.m4a"))
            .await?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn into_song(self) -> Song {
        self.into()
    }
}

impl From<Metronome> for Song {
    fn from(value: Metronome) -> Self {
        Song::new("metronome", Box::new(value))
    }
}

impl Playable for Metronome {
    fn tick(&mut self) -> Result<()> {
        let ctx = self.ctx.clone();
        let sampler = self.sampler.clone();

        self.sequencer
            .tick(self.ctx.current_time(), move |time, _page, step| {
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
}
