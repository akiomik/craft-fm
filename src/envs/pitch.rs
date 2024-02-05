use web_sys::OscillatorNode;

use crate::{result::Result, unit::Frequency};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PitchEnvelope {
    initial: Frequency,
    peak: Frequency,
    attack: f64,
    decay: f64,
    sustain: Frequency,
    release: f64,
    end: Frequency,
}

#[allow(dead_code)]
impl PitchEnvelope {
    pub fn new<I, P, S, E>(
        initial: I,
        peak: P,
        attack: f64,
        decay: f64,
        sustain: S,
        release: f64,
        end: E,
    ) -> Self
    where
        I: Into<Frequency>,
        P: Into<Frequency>,
        S: Into<Frequency>,
        E: Into<Frequency>,
    {
        Self {
            initial: initial.into(),
            peak: peak.into(),
            attack,
            decay,
            sustain: sustain.into(),
            release,
            end: end.into(),
        }
    }

    #[inline]
    pub fn initial(&self) -> Frequency {
        self.initial
    }

    #[inline]
    pub fn peak(&self) -> Frequency {
        self.peak
    }

    #[inline]
    pub fn attack(&self) -> f64 {
        self.attack
    }

    #[inline]
    pub fn decay(&self) -> f64 {
        self.decay
    }

    #[inline]
    pub fn sustain(&self) -> Frequency {
        self.sustain
    }

    #[inline]
    pub fn release(&self) -> f64 {
        self.release
    }

    #[inline]
    pub fn end(&self) -> Frequency {
        self.end
    }

    pub fn attach(&self, src: &mut OscillatorNode, time: f64, duration: f64) -> Result<()> {
        let freq = src.frequency();
        freq.set_value(self.initial.into());
        freq.exponential_ramp_to_value_at_time(self.peak.into(), time + self.attack)?;
        freq.exponential_ramp_to_value_at_time(
            self.sustain.into(),
            time + self.attack + self.decay,
        )?;
        freq.exponential_ramp_to_value_at_time(self.end.into(), time + duration + self.release)?;
        Ok(())
    }
}
