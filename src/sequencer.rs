use wasm_bindgen::prelude::*;
use web_sys::AudioContext;

use crate::interval::Interval;

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
    timer: Option<Interval>,
    interval: u32,
}

impl Sequencer {
    pub fn new(ctx: AudioContext, bpm: usize, resolution: Resolution, interval: u32) -> Self {
        Self {
            ctx,
            bpm,
            resolution,
            interval,
            timer: None,
        }
    }

    pub fn start<F>(&mut self, tick: F) -> Result<(), JsValue>
    where
        F: Fn(f64, usize) -> Result<(), JsValue> + 'static,
    {
        if self.timer.is_some() {
            return Ok(());
        }

        let ctx = self.ctx.clone();
        let beats_per_measure = self.resolution.beats_per_measure();
        let secs = self.seconds_per_beat();
        let interval = self.interval as f64 / 1000.0; // in secs

        let mut beat_time = self.ctx.current_time();
        let mut step = 0;

        let timer = Interval::new(
            move || {
                let time = ctx.current_time();
                let next_time = time + interval;

                while beat_time < next_time {
                    tick(beat_time, step).unwrap(); // TODO
                    beat_time += secs;
                    step = (step + 1) % beats_per_measure;
                }
            },
            self.interval,
        )?;
        self.timer = Some(timer);

        Ok(())
    }

    pub fn stop(&mut self) {
        self.timer = None;
    }

    #[allow(dead_code)]
    pub fn is_playing(&self) -> bool {
        self.timer.is_some()
    }

    pub fn seconds_per_beat(&self) -> f64 {
        (60.0 / self.bpm as f64) * (4.0 / self.resolution.beats_per_measure() as f64)
    }
}
