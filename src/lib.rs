use std::collections::{HashMap, HashSet};

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::AudioBufferSourceOptions;
use web_sys::{js_sys::Uint8Array, AudioBuffer, AudioBufferSourceNode, AudioContext};

mod note;
mod sampler;

use crate::sampler::Sampler;
use crate::note::Note;

async fn load_sample(ctx: &AudioContext, sample: &[u8]) -> Result<AudioBuffer, JsValue> {
    let array_buffer = Uint8Array::from(sample).buffer();
    let decoded = JsFuture::from(ctx.decode_audio_data(&array_buffer)?).await?;
    Ok(AudioBuffer::from(decoded))
}

#[wasm_bindgen]
pub struct Player {
    ctx: AudioContext,
    sampler: Sampler,
    samples: HashMap<Note, AudioBuffer>
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
        let a3 = load_sample(&ctx, include_bytes!("../samples/a3.wav")).await?;
        let mut samples = HashMap::new();
        samples.insert(Note::A3, a3);
        let sampler = Sampler::new(HashSet::from_iter(samples.keys().cloned()));

        Ok(Self { ctx, sampler, samples })
    }

    pub fn play(&self, note: Note) -> Result<(), JsValue> {
        let (note, playback) = self.sampler.calc_playback_at_note(note);
        let buffer = self.samples.get(&note).expect("note not found");

        let mut opts = AudioBufferSourceOptions::new();
        opts.playback_rate(playback);
        let src = AudioBufferSourceNode::new_with_options(&self.ctx, &opts)?;
        src.set_buffer(Some(&buffer));
        src.connect_with_audio_node(&self.ctx.destination())?;
        src.start()?;
        Ok(())
    }
}
