use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use web_sys::AudioContext;

use crate::note::Note;
use crate::sampler::Sampler;
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
        samples.insert(
            Note::A2,
            include_bytes!("../samples/a2.m4a").as_slice().into(),
        );
        samples.insert(
            Note::A3,
            include_bytes!("../samples/a3.m4a").as_slice().into(),
        );
        samples.insert(
            Note::A4,
            include_bytes!("../samples/a4.m4a").as_slice().into(),
        );
        let sampler = Sampler::new(ctx.clone(), samples).await?;
        let sequencer = Sequencer::new(ctx.clone(), 120, 1, Resolution::Eighth, 100)?;

        Ok(Self {
            ctx,
            sampler,
            sequencer,
        })
    }

    pub fn play(&mut self, note: Note) -> Result<(), JsValue> {
        self.sequencer.stop()?;

        let ctx = self.ctx.clone();
        let sampler = self.sampler.clone();

        self.sequencer.start(move |time, step, _page| {
            if step == 0 {
                let src = sampler.buffer_node(&Note::C4)?;
                src.connect_with_audio_node(&ctx.destination())?;
                src.start_with_when(time)?;
            } else {
                let src = sampler.buffer_node(&note)?;
                src.connect_with_audio_node(&ctx.destination())?;
                src.start_with_when(time)?;
            }
            Ok(())
        })?;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), JsValue> {
        self.sequencer.stop()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    use super::*;

    #[wasm_bindgen_test]
    async fn test_play() {
        let mut player = Player::new().await.unwrap();
        assert_eq!(player.sequencer.is_playing(), false);
        player.play(Note::A4).unwrap();
        assert_eq!(player.sequencer.is_playing(), true);
    }

    #[wasm_bindgen_test]
    async fn test_stop() {
        let mut player = Player::new().await.unwrap();
        assert_eq!(player.sequencer.is_playing(), false);
        player.stop().unwrap();
        assert_eq!(player.sequencer.is_playing(), false);
    }

    #[wasm_bindgen_test]
    async fn test_play_and_stop() {
        let mut player = Player::new().await.unwrap();
        assert_eq!(player.sequencer.is_playing(), false);
        player.play(Note::A4).unwrap();
        assert_eq!(player.sequencer.is_playing(), true);
        player.stop().unwrap();
        assert_eq!(player.sequencer.is_playing(), false);
    }
}
