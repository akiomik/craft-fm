use wasm_bindgen::JsValue;
use web_sys::{AudioContext, BiquadFilterNode};

use crate::{envelope::AmpEnvelope, theory::Note};

#[derive(Debug, Clone)]
pub struct Toy808 {
    ctx: AudioContext,
}

#[allow(dead_code)]
impl Toy808 {
    pub fn new(ctx: AudioContext) -> Self {
        Self { ctx }
    }

    pub fn bd(&self, time: f64) -> Result<BiquadFilterNode, JsValue> {
        let volume = 1.0;
        let duration = 0.125;
        let attack = 0.003;
        let decay = 0.002;
        let cutoff = 4000.0;

        let osc = self.ctx.create_oscillator()?;
        osc.set_type(web_sys::OscillatorType::Sine);
        osc.start_with_when(time)?;
        osc.stop_with_when(time + duration)?;

        let freq = osc.frequency();
        freq.set_value(Note::A1.freq());
        freq.exponential_ramp_to_value_at_time(Note::A2.freq(), time + attack)?;
        freq.exponential_ramp_to_value_at_time(Note::A1.freq(), time + attack + decay)?;

        let amp_env = AmpEnvelope::new(self.ctx.clone(), volume, attack, decay, 0.0, 0.0);
        let amp = amp_env.node(&osc, time, duration)?;

        let filter = self.ctx.create_biquad_filter()?;
        filter.q().set_value(1.0);
        filter.set_type(web_sys::BiquadFilterType::Lowpass);
        filter.frequency().set_value(cutoff);

        amp.connect_with_audio_node(&filter)?;
        Ok(filter)
    }
}
