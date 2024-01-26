use std::collections::HashMap;

use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{js_sys::Uint8Array, JsFuture};
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioBufferSourceOptions, AudioContext};

use crate::note::Note;

#[derive(Clone)]
pub struct Sampler {
    ctx: AudioContext,
    samples: HashMap<Note, AudioBuffer>,
}

impl Sampler {
    pub async fn new(
        ctx: AudioContext,
        samples: HashMap<Note, Box<[u8]>>,
    ) -> Result<Self, JsValue> {
        let mut buffered_samples = HashMap::new();
        for (note, sample) in samples.iter() {
            let buffer = Sampler::buffer(&ctx, sample).await?;
            buffered_samples.insert(note.clone(), buffer);
        }

        Ok(Self {
            ctx,
            samples: buffered_samples,
        })
    }

    async fn buffer(ctx: &AudioContext, sample: &[u8]) -> Result<AudioBuffer, JsValue> {
        let array_buffer = Uint8Array::from(sample).buffer();
        let decoded = JsFuture::from(ctx.decode_audio_data(&array_buffer)?).await?;
        Ok(AudioBuffer::from(decoded))
    }

    fn calc_note_and_playback_rate(&self, note: &Note) -> (Note, f32) {
        if self.samples.contains_key(note) {
            (note.clone(), 1.0)
        } else {
            // TODO: find closest note from notes
            (Note::A3, note.freq() / Note::A3.freq())
        }
    }

    pub fn buffer_node(&self, note: &Note) -> Result<AudioBufferSourceNode, JsValue> {
        let (sample_note, playback_rate) = self.calc_note_and_playback_rate(note);
        let buffer = self.samples.get(&sample_note).expect("note not found");

        let mut opts = AudioBufferSourceOptions::new();
        opts.playback_rate(playback_rate);

        let src = AudioBufferSourceNode::new_with_options(&self.ctx, &opts)?;
        src.set_buffer(Some(buffer));
        Ok(src)
    }
}
