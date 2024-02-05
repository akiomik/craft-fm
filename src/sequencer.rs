use crate::{result::Result, theory::Duration};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Resolution {
    Eighth,
    Quarter,
}

impl Resolution {
    pub fn duration(&self) -> Duration {
        match self {
            Resolution::Eighth => Duration::Eighth,
            Resolution::Quarter => Duration::Quarter,
        }
    }
}

pub struct Sequencer {
    bpm: f32,
    pages: usize,
    resolution: Resolution,
    interval: u32,
    step: usize,
    page: usize,
    beat_time: f64,
}

impl Sequencer {
    pub fn new(
        bpm: f32,
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

    pub fn tick<F>(&mut self, current_time: f64, mut f: F) -> Result<()>
    where
        F: FnMut(f64, usize, usize) -> Result<()>,
    {
        let beats_per_measure = self.resolution.duration().beats_per_measure();
        let seconds_per_beat = self.seconds_per_beat();
        let interval = self.interval as f64 / 1000.0; // in secs

        let next_time = current_time + interval;
        while self.beat_time < next_time {
            // NOTE: Added interval as an offset for the first beat
            f(self.beat_time + interval, self.page, self.step)?;

            self.beat_time += seconds_per_beat;
            self.step = (self.step + 1) % beats_per_measure;
            if self.step == 0 {
                self.page = (self.page + 1) % self.pages;
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    #[inline]
    pub fn resolution(&self) -> Resolution {
        self.resolution.clone()
    }

    #[inline]
    pub fn seconds_per_beat(&self) -> f64 {
        (60.0 / self.bpm as f64) * (4.0 * self.resolution.duration().relative() as f64)
    }

    #[allow(dead_code)]
    #[inline]
    pub fn set_bpm(&mut self, bpm: f32) {
        self.bpm = bpm;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_quarter() {
        let mut current_time = 0.0;
        let mut seq = Sequencer::new(60.0, 2, Resolution::Quarter, current_time, 100);

        for i in 0..2 {
            for j in 0..4 {
                seq.tick(current_time, |time, page, step| {
                    assert_eq!(time, current_time + 0.1);
                    assert_eq!(page, i);
                    assert_eq!(step, j);
                    Ok(())
                })
                .unwrap();
                current_time += 1.0;
            }
        }
    }

    #[test]
    fn test_tick_eighth() {
        let mut current_time = 0.0;
        let mut seq = Sequencer::new(60.0, 2, Resolution::Eighth, current_time, 100);

        for i in 0..2 {
            for j in 0..8 {
                seq.tick(current_time, |time, page, step| {
                    assert_eq!(time, current_time + 0.1);
                    assert_eq!(page, i);
                    assert_eq!(step, j);
                    Ok(())
                })
                .unwrap();
                current_time += 0.5;
            }
        }
    }

    #[test]
    fn test_seconds_per_beat_60_4() {
        let seq = Sequencer::new(60.0, 1, Resolution::Quarter, 0.0, 100);
        assert_eq!(seq.seconds_per_beat(), 1.0);
    }

    #[test]
    fn test_seconds_per_beat_60_8() {
        let seq = Sequencer::new(60.0, 1, Resolution::Eighth, 0.0, 100);
        assert_eq!(seq.seconds_per_beat(), 0.5);
    }

    #[test]
    fn test_seconds_per_beat_120_4() {
        let seq = Sequencer::new(120.0, 1, Resolution::Quarter, 0.0, 100);
        assert_eq!(seq.seconds_per_beat(), 0.5);
    }

    #[test]
    fn test_seconds_per_beat_120_8() {
        let seq = Sequencer::new(120.0, 1, Resolution::Eighth, 0.0, 100);
        assert_eq!(seq.seconds_per_beat(), 0.25);
    }
}
