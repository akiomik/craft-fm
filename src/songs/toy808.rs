use wasm_bindgen::prelude::*;
use web_sys::AudioContext;

use crate::{
    machines,
    sequencer::{Resolution, Sequencer},
};

use super::{Playable, Song};

#[wasm_bindgen]
pub struct Toy808 {
    ctx: AudioContext,
    machine: machines::Toy808,
    sequencer: Sequencer,
}

#[wasm_bindgen]
impl Toy808 {
    #[wasm_bindgen(constructor)]
    pub fn new(ctx: AudioContext, bpm: f32) -> Result<Toy808, JsValue> {
        let machine = machines::Toy808::new(ctx.clone());
        let sequencer = Sequencer::new(bpm, 1, Resolution::Quarter, ctx.current_time(), 100);

        Ok(Self {
            ctx,
            machine,
            sequencer,
        })
    }

    #[wasm_bindgen]
    pub fn into_song(self) -> Song {
        self.into()
    }
}

impl From<Toy808> for Song {
    fn from(value: Toy808) -> Self {
        Song::new("toy808", Box::new(value))
    }
}

impl Playable for Toy808 {
    fn tick(&mut self) -> Result<(), JsValue> {
        let ctx = self.ctx.clone();
        let machine = self.machine.clone();

        self.sequencer
            .tick(self.ctx.current_time(), move |time, _page, step| {
                let src = machine.bd(time)?;
                src.connect_with_audio_node(&ctx.destination())?;

                Ok(())
            })
    }
}
