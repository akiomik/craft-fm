use std::{cell::RefCell, collections::HashMap, rc::Rc};

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use wasm_bindgen::prelude::*;
use web_sys::AudioContext;

use crate::{
    arps::UpDownArpeggiator,
    chord::Chord,
    note::Note,
    sampler::MelodicSampler,
    sequencer::{Resolution, Sequencer},
};

use super::{Playable, Song};

#[wasm_bindgen]
pub struct Forest {
    ctx: AudioContext,
    sampler: MelodicSampler,
    sequencer: Sequencer,
    rng: Rc<RefCell<ChaCha8Rng>>,
}

#[wasm_bindgen]
impl Forest {
    #[wasm_bindgen(constructor)]
    pub async fn new(ctx: AudioContext, seed: u64) -> Result<Forest, JsValue> {
        let mut samples = HashMap::new();
        samples.insert(
            Note::A0,
            include_bytes!("../../samples/a0.m4a").as_slice().into(),
        );
        samples.insert(
            Note::A1,
            include_bytes!("../../samples/a1.m4a").as_slice().into(),
        );
        samples.insert(
            Note::A2,
            include_bytes!("../../samples/a2.m4a").as_slice().into(),
        );
        samples.insert(
            Note::A3,
            include_bytes!("../../samples/a3.m4a").as_slice().into(),
        );
        samples.insert(
            Note::A4,
            include_bytes!("../../samples/a4.m4a").as_slice().into(),
        );
        let sampler = MelodicSampler::new(ctx.clone(), samples).await?;
        let sequencer = Sequencer::new(ctx.clone(), 74, 8, Resolution::Eighth, 100)?;
        let rng = Rc::new(RefCell::new(ChaCha8Rng::seed_from_u64(seed)));

        Ok(Self {
            ctx,
            sampler,
            sequencer,
            rng,
        })
    }

    // NOTE: Can't use `impl Into<Song>` and `impl From<Forest>` on js
    #[wasm_bindgen]
    pub fn into_song(self) -> Song {
        Song::new("forest", Box::new(self))
    }
}

impl Playable for Forest {
    fn play(&mut self) -> Result<(), JsValue> {
        let ctx = self.ctx.clone();
        let sampler = self.sampler.clone();
        let rng_ref = self.rng.clone();
        let beats_per_measure = self.sequencer.resolution().beats_per_measure();

        let lhs_chords: [Vec<Note>; 2] = [
            UpDownArpeggiator::new(Chord::Major9th(Note::G1).notes())
                .take(beats_per_measure)
                .collect(),
            UpDownArpeggiator::new(Chord::Major9th(Note::C1).notes())
                .take(beats_per_measure)
                .collect(),
        ];
        let rhs_chords = [
            Chord::Major9th(Note::G3).notes(),
            Chord::Major9th(Note::C3).notes(),
        ];

        self.sequencer.start(move |time, step, page| {
            let chord_index = if page >= 4 { 1 } else { 0 };

            // left hand
            if page % 2 == 0 || step < 4 || page == 7 {
                let chord = lhs_chords
                    .get(chord_index)
                    .expect("should be got chord from chords");
                let note = chord.get(step).expect("should be got note from chord");
                let src = sampler.buffer_node(note)?;
                src.connect_with_audio_node(&ctx.destination())?;
                src.start_with_when(time)?;
            }

            // right hand
            let mut rng = rng_ref.borrow_mut();
            if step == 0
                || (step == 6 && rng.gen_range(0..3) == 0)
                || (page == 7 && step == 6)
                || (page % 2 == 1 && page != 7 && step == 7)
            {
                let chord = rhs_chords
                    .get(chord_index)
                    .expect("should be got chord from chords");
                let note_index = rng.gen_range(0..chord.len());
                let note = chord
                    .get(note_index)
                    .expect("should be got note from chord");
                let src = sampler.buffer_node(note)?;
                src.connect_with_audio_node(&ctx.destination())?;
                src.start_with_when(time)?;
            }

            Ok(())
        })
    }

    fn stop(&mut self) -> Result<(), JsValue> {
        self.sequencer.stop()
    }
}
