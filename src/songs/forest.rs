use std::{cell::RefCell, rc::Rc};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use wasm_bindgen::prelude::*;
use web_sys::AudioContext;

use crate::{
    arps::UpDownArpeggiator,
    result::Result,
    sampler::MelodicSampler,
    sequencer::{Resolution, Sequencer},
    theory::*,
};

use super::{Playable, Song};

#[wasm_bindgen]
pub struct Forest {
    ctx: AudioContext,
    sampler: MelodicSampler,
    sequencer: Sequencer,
    rng: Rc<RefCell<ChaCha8Rng>>,
    lhs_chords: Vec<Vec<Note>>,
    rhs_chords: Vec<Vec<Note>>,
}

#[wasm_bindgen]
impl Forest {
    #[wasm_bindgen(constructor)]
    pub fn new(ctx: AudioContext, seed: u64) -> Forest {
        let sequencer = Sequencer::new(74.0, 8, Resolution::Eighth, ctx.current_time(), 100);
        let rng = Rc::new(RefCell::new(ChaCha8Rng::seed_from_u64(seed)));

        let beats_per_measure = sequencer.resolution().duration().beats_per_measure();
        let lhs_chords = vec![
            UpDownArpeggiator::new(Chord::Major9th(Note::G1).notes())
                .take(beats_per_measure)
                .collect(),
            UpDownArpeggiator::new(Chord::Major9th(Note::C1).notes())
                .take(beats_per_measure)
                .collect(),
        ];
        let rhs_chords = vec![
            Chord::Major9th(Note::G3).notes(),
            Chord::Major9th(Note::C3).notes(),
        ];

        Self {
            ctx: ctx.clone(),
            sampler: MelodicSampler::new(ctx),
            sequencer,
            rng,
            lhs_chords,
            rhs_chords,
        }
    }

    #[wasm_bindgen]
    pub async fn init(&mut self) -> Result<()> {
        self.sampler
            .insert(Note::A0, include_bytes!("../../samples/a0.m4a"))
            .await?;
        self.sampler
            .insert(Note::A1, include_bytes!("../../samples/a1.m4a"))
            .await?;
        self.sampler
            .insert(Note::A2, include_bytes!("../../samples/a2.m4a"))
            .await?;
        self.sampler
            .insert(Note::A3, include_bytes!("../../samples/a3.m4a"))
            .await?;
        self.sampler
            .insert(Note::A4, include_bytes!("../../samples/a4.m4a"))
            .await?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn into_song(self) -> Song {
        self.into()
    }
}

impl From<Forest> for Song {
    fn from(value: Forest) -> Self {
        Song::new("forest", Box::new(value))
    }
}

impl Playable for Forest {
    fn tick(&mut self) -> Result<()> {
        let ctx = self.ctx.clone();
        let sampler = self.sampler.clone();
        let rng_ref = self.rng.clone();
        let lhs_chords = self.lhs_chords.clone();
        let rhs_chords = self.rhs_chords.clone();

        self.sequencer
            .tick(ctx.current_time(), move |time, page, step| {
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
                    || (step == 6 && rng.random_range(0..3) == 0)
                    || (page == 7 && step == 6)
                    || (page % 2 == 1 && page != 7 && step == 7)
                {
                    let chord = rhs_chords
                        .get(chord_index)
                        .expect("should be got chord from chords");
                    let note_index = rng.random_range(0..chord.len());
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
}
