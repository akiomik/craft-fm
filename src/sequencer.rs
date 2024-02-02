use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
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
    bpm: usize,
    pages: usize,
    resolution: Resolution,
    interval: u32,
    step: usize,
    page: usize,
    beat_time: f64,
}

impl Sequencer {
    pub fn new(
        bpm: usize,
        pages: usize,
        resolution: Resolution,
        current_time: f64,
        interval: u32,
    ) -> Self {
        Self {
            bpm,
            pages,
            resolution,
            interval,
            step: 0,
            page: 0,
            beat_time: current_time,
        }
    }

    pub fn tick<F>(&mut self, current_time: f64, mut f: F) -> Result<(), JsValue>
    where
        F: FnMut(f64, usize, usize) -> Result<(), JsValue>,
    {
        let beats_per_measure = self.resolution.beats_per_measure();
        let interval = self.interval as f64 / 1000.0; // in secs

        let next_time = current_time + interval;
        println!("beat_time = {}, next_time = {next_time}", self.beat_time);
        while self.beat_time < next_time {
            // NOTE: Added interval as an offset for the first beat
            f(self.beat_time + interval, self.step, self.page).expect("tick should succeed");

            self.beat_time += self.seconds_per_beat();
            self.step = (self.step + 1) % beats_per_measure;
            if self.step == 0 {
                self.page = (self.page + 1) % self.pages;
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn resolution(&self) -> Resolution {
        self.resolution.clone()
    }

    fn seconds_per_beat(&self) -> f64 {
        (60.0 / self.bpm as f64) * (4.0 / self.resolution.beats_per_measure() as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_quarter() {
        let mut time = 0.0;
        let mut seq = Sequencer::new(60, 2, Resolution::Quarter, time, 100);

        for i in 0..2 {
            for j in 0..4 {
                seq.tick(time, |_time, step, page| {
                    println!("i = {i}, j = {j}, page = {page}, step = {step}");
                    assert_eq!(page, i);
                    assert_eq!(step, j);
                    Ok(())
                })
                .unwrap();
                time += 1.0;
            }
        }
    }

    #[test]
    fn test_tick_eighth() {
        let mut time = 0.0;
        let mut seq = Sequencer::new(60, 2, Resolution::Eighth, time, 100);

        for i in 0..2 {
            for j in 0..8 {
                seq.tick(time, |_time, step, page| {
                    println!("i = {i}, j = {j}, page = {page}, step = {step}");
                    assert_eq!(page, i);
                    assert_eq!(step, j);
                    Ok(())
                })
                .unwrap();
                time += 0.5;
            }
        }
    }

    #[test]
    fn test_seconds_per_beat_60_4() {
        let seq = Sequencer::new(60, 1, Resolution::Quarter, 0.0, 100);
        assert_eq!(seq.seconds_per_beat(), 1.0);
    }

    #[test]
    fn test_seconds_per_beat_60_8() {
        let seq = Sequencer::new(60, 1, Resolution::Eighth, 0.0, 100);
        assert_eq!(seq.seconds_per_beat(), 0.5);
    }

    #[test]
    fn test_seconds_per_beat_120_4() {
        let seq = Sequencer::new(120, 1, Resolution::Quarter, 0.0, 100);
        assert_eq!(seq.seconds_per_beat(), 0.5);
    }

    #[test]
    fn test_seconds_per_beat_120_8() {
        let seq = Sequencer::new(120, 1, Resolution::Eighth, 0.0, 100);
        assert_eq!(seq.seconds_per_beat(), 0.25);
    }
}
