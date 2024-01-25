use std::{collections::HashMap};

use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{js_sys::Uint8Array, JsFuture};
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioBufferSourceOptions, AudioContext};

use crate::note::Note;

pub struct Sampler {
    ctx: AudioContext,
    samples: HashMap<Note, Box<[u8]>>,
}

impl Sampler {
    pub fn new(ctx: AudioContext, samples: HashMap<Note, Box<[u8]>>) -> Self {
        Self { ctx, samples }
    }

    fn calc_note_and_playback_rate(&self, note: &Note) -> (Note, f32) {
        if self.samples.contains_key(note) {
            (note.clone(), 1.0)
        } else {
            // TODO: find closest note from notes
            (Note::A3, note.freq() / Note::A3.freq())
        }
    }

    async fn buffer(&self, note: &Note) -> Result<AudioBuffer, JsValue> {
        let sample = self.samples.get(note).expect("note not found");
        let array_buffer = Uint8Array::from(sample.as_ref()).buffer();
        let decoded = JsFuture::from(self.ctx.decode_audio_data(&array_buffer)?).await?;
        Ok(AudioBuffer::from(decoded))
    }

    pub async fn buffer_node(&self, note: &Note) -> Result<AudioBufferSourceNode, JsValue> {
        let (sample_note, playback_rate) = self.calc_note_and_playback_rate(note);
        let buffer = self.buffer(&sample_note).await?;

        let mut opts = AudioBufferSourceOptions::new();
        opts.playback_rate(playback_rate);

        let src = AudioBufferSourceNode::new_with_options(&self.ctx, &opts)?;
        src.set_buffer(Some(&buffer));
        Ok(src)
    }
}
