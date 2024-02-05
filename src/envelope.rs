use web_sys::{AudioContext, AudioNode, GainNode};

use crate::result::Result;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AmpEnvelope {
    ctx: AudioContext,
    volume: f32,
    attack: f64,
    decay: f64,
    sustain: f32,
    release: f64,
}

#[allow(dead_code)]
impl AmpEnvelope {
    pub fn new(
        ctx: AudioContext,
        volume: f32,
        attack: f64,
        decay: f64,
        sustain: f32,
        release: f64,
    ) -> Self {
        Self {
            ctx,
            volume,
            attack,
            decay,
            sustain,
            release,
        }
    }

    #[inline]
    pub fn volume(&self) -> f32 {
        self.volume
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
    pub fn sustain(&self) -> f32 {
        self.sustain
    }

    #[inline]
    pub fn release(&self) -> f64 {
        self.release
    }

    pub fn node(&self, src: &AudioNode, time: f64, duration: f64) -> Result<GainNode> {
        let gain = self.ctx.create_gain()?;
        let param = gain.gain();
        param.set_value(0.0);
        param.linear_ramp_to_value_at_time(self.volume, time + self.attack)?;
        param.linear_ramp_to_value_at_time(self.sustain, time + self.attack + self.decay)?;
        param.linear_ramp_to_value_at_time(0.0, time + duration + self.release)?;
        src.connect_with_audio_node(&gain)?;

        Ok(gain)
    }
}
