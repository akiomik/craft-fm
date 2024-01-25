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
        let res = self.resolution.clone();
        let secs = self.seconds_per_beat();
        let timer = Interval::new(
            move || {
                // TODO: setIntervalの間隔によらずtimeベースでtickを実行する
                let time = ctx.current_time();
                for step in 0..res.beats_per_measure() {
                    let offset = secs * step as f64;
                    tick(time + offset, step).unwrap(); // TODO
                }
            },
            self.interval,
        );
        self.timer = Some(timer);

        Ok(())
    }

    pub fn stop(&mut self) {
        self.timer = None;
    }

    pub fn seconds_per_beat(&self) -> f64 {
        (60.0 / self.bpm as f64) * (4.0 / self.resolution.beats_per_measure() as f64)
    }
}
