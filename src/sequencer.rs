use wasm_bindgen::prelude::*;
use web_sys::AudioContext;

use crate::worker::WebWorker;

#[allow(dead_code)]
#[derive(Clone)]
pub enum Resolution {
    Quarter,
    Eighth,
}

impl Resolution {
    pub fn beats_per_measure(&self) -> usize {
        match self {
            Resolution::Quarter => 4,
            Resolution::Eighth => 8,
        }
    }
}

pub struct Sequencer {
    ctx: AudioContext,
    bpm: usize,
    pages: usize,
    resolution: Resolution,
    interval: u32,
    worker: WebWorker,
    is_playing: bool,
}

impl Sequencer {
    pub fn new(
        ctx: AudioContext,
        bpm: usize,
        pages: usize,
        resolution: Resolution,
        interval: u32,
    ) -> Result<Self, JsValue> {
        let worker = WebWorker::new("./worker.js")?;

        Ok(Self {
            ctx,
            bpm,
            pages,
            resolution,
            interval,
            worker,
            is_playing: false,
        })
    }

    pub fn start<F>(&mut self, tick: F) -> Result<(), JsValue>
    where
        F: Fn(f64, usize, usize) -> Result<(), JsValue> + 'static,
    {
        if self.is_playing() {
            return Ok(());
        }

        let ctx = self.ctx.clone();
        let beats_per_measure = self.resolution.beats_per_measure();
        let secs = self.seconds_per_beat();
        let pages = self.pages;
        let interval = self.interval as f64 / 1000.0; // in secs

        let mut beat_time = ctx.current_time();
        let mut page = 0;
        let mut step = 0;

        self.worker.set_onmessage(move |message| {
            if message.data() == "tick" {
                let next_time = ctx.current_time() + interval;

                while beat_time < next_time {
                    // NOTE: Added interval as an offset for the first beat
                    tick(beat_time + interval, step, page).expect("tick should succeed");
                    beat_time += secs;
                    step = (step + 1) % beats_per_measure;
                    if step == 0 {
                        page = (page + 1) % pages;
                    }
                }
            }
        });
        self.worker.post_message("start")?;
        self.is_playing = true;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), JsValue> {
        self.worker.post_message("stop")?;
        self.is_playing = false;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    fn seconds_per_beat(&self) -> f64 {
        (60.0 / self.bpm as f64) * (4.0 / self.resolution.beats_per_measure() as f64)
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    use super::*;

    #[wasm_bindgen_test]
    fn test_start() {
        let ctx = AudioContext::new().unwrap();
        let mut seq = Sequencer::new(ctx, 60, 1, Resolution::Quarter, 100).unwrap();
        assert_eq!(seq.is_playing(), false);
        seq.start(|_, _, _| Ok(())).unwrap();
        assert_eq!(seq.is_playing(), true);
    }

    #[wasm_bindgen_test]
    fn test_stop() {
        let ctx = AudioContext::new().unwrap();
        let mut seq = Sequencer::new(ctx, 60, 1, Resolution::Quarter, 100).unwrap();
        assert_eq!(seq.is_playing(), false);
        seq.stop().unwrap();
        assert_eq!(seq.is_playing(), false);
    }

    #[wasm_bindgen_test]
    fn test_start_and_stop() {
        let ctx = AudioContext::new().unwrap();
        let mut seq = Sequencer::new(ctx, 60, 1, Resolution::Quarter, 100).unwrap();
        assert_eq!(seq.is_playing(), false);
        seq.start(|_, _, _| Ok(())).unwrap();
        assert_eq!(seq.is_playing(), true);
        seq.stop().unwrap();
        assert_eq!(seq.is_playing(), false);
    }

    #[wasm_bindgen_test]
    fn test_seconds_per_beat_60_4() {
        let ctx = AudioContext::new().unwrap();
        let seq = Sequencer::new(ctx, 60, 1, Resolution::Quarter, 100).unwrap();
        assert_eq!(seq.seconds_per_beat(), 1.0);
    }

    #[wasm_bindgen_test]
    fn test_seconds_per_beat_60_8() {
        let ctx = AudioContext::new().unwrap();
        let seq = Sequencer::new(ctx, 60, 1, Resolution::Eighth, 100).unwrap();
        assert_eq!(seq.seconds_per_beat(), 0.5);
    }

    #[wasm_bindgen_test]
    fn test_seconds_per_beat_120_4() {
        let ctx = AudioContext::new().unwrap();
        let seq = Sequencer::new(ctx, 120, 1, Resolution::Quarter, 100).unwrap();
        assert_eq!(seq.seconds_per_beat(), 0.5);
    }

    #[wasm_bindgen_test]
    fn test_seconds_per_beat_120_8() {
        let ctx = AudioContext::new().unwrap();
        let seq = Sequencer::new(ctx, 120, 1, Resolution::Eighth, 100).unwrap();
        assert_eq!(seq.seconds_per_beat(), 0.25);
    }
}
