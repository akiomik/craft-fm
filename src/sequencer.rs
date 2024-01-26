use wasm_bindgen::prelude::*;
use web_sys::AudioContext;

use crate::worker::WebWorker;

#[derive(Clone)]
pub enum Resolution {
    Eighth,
}

impl Resolution {
    pub fn beats_per_measure(&self) -> usize {
        match self {
            Resolution::Eighth => 8,
        }
    }
}

pub struct Sequencer {
    ctx: AudioContext,
    bpm: usize,
    resolution: Resolution,
    interval: u32,
    worker: WebWorker,
    is_playing: bool,
}

impl Sequencer {
    pub fn new(
        ctx: AudioContext,
        bpm: usize,
        resolution: Resolution,
        interval: u32,
    ) -> Result<Self, JsValue> {
        let worker = WebWorker::new("./worker.js")?;

        Ok(Self {
            ctx,
            bpm,
            resolution,
            interval,
            worker,
            is_playing: false,
        })
    }

    pub fn start<F>(&mut self, tick: F) -> Result<(), JsValue>
    where
        F: Fn(f64, usize) -> Result<(), JsValue> + 'static,
    {
        if self.is_playing() {
            return Ok(());
        }

        let ctx = self.ctx.clone();
        let beats_per_measure = self.resolution.beats_per_measure();
        let secs = self.seconds_per_beat();
        let interval = self.interval as f64 / 1000.0; // in secs

        let mut beat_time = ctx.current_time();
        let mut step = 0;

        self.worker.set_onmessage(move |message| {
            if message.data() == "tick" {
                let next_time = ctx.current_time() + interval;

                while beat_time < next_time {
                    // NOTE: Added interval as an offset for the first beat
                    tick(beat_time + interval, step).expect("tick should succeed");
                    beat_time += secs;
                    step = (step + 1) % beats_per_measure;
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

    pub fn seconds_per_beat(&self) -> f64 {
        (60.0 / self.bpm as f64) * (4.0 / self.resolution.beats_per_measure() as f64)
    }
}
