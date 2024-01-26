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
        // TODO: check if samples is not empty
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

    fn find_closest_note_in_samples(&self, note: &Note) -> Option<Note> {
        if self.samples.is_empty() {
            return None;
        }

        let note_number = note.note_number() as i16;
        let closest_note = self
            .samples
            .keys()
            .fold((Note::A4, i16::MAX), |(acc_key, acc_diff), key| {
                let diff = (note_number - key.note_number() as i16).abs();
                if acc_diff > diff {
                    (key.clone(), diff)
                } else {
                    (acc_key, acc_diff)
                }
            })
            .0;
        Some(closest_note)
    }

    fn calc_note_and_playback_rate(&self, note: &Note) -> (Note, f32) {
        if self.samples.contains_key(note) {
            (note.clone(), 1.0)
        } else {
            let closest_note = self
                .find_closest_note_in_samples(note)
                .expect("closest note should be found"); // TODO
            let closest_freq = closest_note.freq();
            (closest_note, note.freq() / closest_freq)
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

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    use super::*;

    const A2: &[u8] = include_bytes!("../samples/a2.m4a").as_slice();
    const A3: &[u8] = include_bytes!("../samples/a3.m4a").as_slice();

    #[wasm_bindgen_test]
    async fn test_find_closest_note_in_samples_0() {
        let ctx = AudioContext::new().unwrap();
        let samples = HashMap::new();
        let sampler = Sampler::new(ctx, samples).await.unwrap();
        assert_eq!(sampler.find_closest_note_in_samples(&Note::A2), None);
    }

    #[wasm_bindgen_test]
    async fn test_find_closest_note_in_samples_1_contains() {
        let ctx = AudioContext::new().unwrap();
        let mut samples = HashMap::new();
        samples.insert(Note::A2, A2.into());
        let sampler = Sampler::new(ctx, samples).await.unwrap();
        assert_eq!(
            sampler.find_closest_note_in_samples(&Note::A2),
            Some(Note::A2)
        );
    }

    #[wasm_bindgen_test]
    async fn test_find_closest_note_in_samples_1_not_contains() {
        let ctx = AudioContext::new().unwrap();
        let mut samples = HashMap::new();
        samples.insert(Note::A2, A2.into());
        let sampler = Sampler::new(ctx, samples).await.unwrap();
        assert_eq!(
            sampler.find_closest_note_in_samples(&Note::C2),
            Some(Note::A2)
        );
    }

    #[wasm_bindgen_test]
    async fn test_find_closest_note_in_samples_2_contains() {
        let ctx = AudioContext::new().unwrap();
        let mut samples = HashMap::new();
        samples.insert(Note::A2, A2.into());
        samples.insert(Note::A3, A3.into());
        let sampler = Sampler::new(ctx, samples).await.unwrap();
        assert_eq!(
            sampler.find_closest_note_in_samples(&Note::A2),
            Some(Note::A2)
        );
        assert_eq!(
            sampler.find_closest_note_in_samples(&Note::A3),
            Some(Note::A3)
        );
    }

    #[wasm_bindgen_test]
    async fn test_find_closest_note_in_samples_2_not_contains() {
        let ctx = AudioContext::new().unwrap();
        let mut samples = HashMap::new();
        samples.insert(Note::A2, A2.into());
        samples.insert(Note::A3, A3.into());
        let sampler = Sampler::new(ctx, samples).await.unwrap();
        assert_eq!(
            sampler.find_closest_note_in_samples(&Note::C2),
            Some(Note::A2)
        );
        assert_eq!(
            sampler.find_closest_note_in_samples(&Note::C3),
            Some(Note::A2)
        );
        assert_eq!(
            sampler.find_closest_note_in_samples(&Note::C4),
            Some(Note::A3)
        );
    }
}
