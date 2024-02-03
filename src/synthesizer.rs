use wasm_bindgen::JsValue;
use web_sys::{AudioContext, GainNode, OscillatorType};

use crate::{envelope::AmpEnvelope, theory::Note};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Synthesizer {
    ctx: AudioContext,
    amp: AmpEnvelope,
    shape: OscillatorType,
}

#[allow(dead_code)]
impl Synthesizer {
    pub fn new(ctx: AudioContext, amp: AmpEnvelope, shape: OscillatorType) -> Self {
        Self { ctx, amp, shape }
    }

    pub fn node(&self, note: &Note, time: f64, duration: f64) -> Result<GainNode, JsValue> {
        let osc = self.ctx.create_oscillator()?;
        osc.set_type(self.shape);
        osc.frequency().set_value(note.freq());
        osc.start_with_when(time)?;
        osc.stop_with_when(time + duration + self.amp.release())?;

        let gain = self.amp.node(&osc, time, duration)?;
        Ok(gain)
    }
}
