use web_sys::{AudioContext, GainNode, OscillatorType};

use crate::{
    envs::{AmpEnvelope, PitchEnvelope},
    result::Result,
    theory::Note,
};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Synthesizer {
    ctx: AudioContext,
    shape: OscillatorType,
    amp: AmpEnvelope,
    pitch: Option<PitchEnvelope>,
}

#[allow(dead_code)]
impl Synthesizer {
    pub fn new(
        ctx: AudioContext,
        shape: OscillatorType,
        amp: AmpEnvelope,
        pitch: Option<PitchEnvelope>,
    ) -> Self {
        Self {
            ctx,
            shape,
            amp,
            pitch,
        }
    }

    pub fn node(&self, note: &Note, time: f64, duration: f64) -> Result<GainNode> {
        let mut osc = self.ctx.create_oscillator()?;
        osc.set_type(self.shape);

        if let Some(ref pitch) = self.pitch {
            // TODO: note support
            pitch.attach(&mut osc, time, duration)?;
        } else {
            osc.frequency().set_value(note.freq().into());
        }

        osc.start_with_when(time)?;
        osc.stop_with_when(time + duration + self.amp.release())?;

        let gain = self.amp.node(&osc, time, duration)?;
        Ok(gain)
    }
}
