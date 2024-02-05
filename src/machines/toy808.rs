use web_sys::{AudioContext, BiquadFilterNode, GainNode};

use crate::{envs::AmpEnvelope, noise::Noise, result::Result, theory::Note};

#[derive(Debug, Clone)]
pub struct Toy808 {
    ctx: AudioContext,
}

#[allow(dead_code)]
impl Toy808 {
    pub fn new(ctx: AudioContext) -> Self {
        Self { ctx }
    }

    pub fn bd(&self, time: f64) -> Result<BiquadFilterNode> {
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

    pub fn sd(&self, time: f64) -> Result<GainNode> {
        let volume = 0.25;
        let noise_volume = volume * 0.2;
        let duration = 0.125;
        let attack = 0.01;
        let decay = 0.08;
        let osc_cutoff = 450.0;
        let noise_cutoff = 1000.0;

        let low_osc = self.ctx.create_oscillator()?;
        low_osc.set_type(web_sys::OscillatorType::Sine);
        low_osc.frequency().set_value(Note::C2.freq());
        low_osc.start_with_when(time)?;
        low_osc.stop_with_when(time + duration)?;

        let low_freq = low_osc.frequency();
        low_freq.set_value(Note::C2.freq());
        low_freq.exponential_ramp_to_value_at_time(Note::C3.freq(), time + attack)?;
        low_freq.exponential_ramp_to_value_at_time(Note::C2.freq(), time + attack + decay * 2.0)?;

        let high_osc = self.ctx.create_oscillator()?;
        high_osc.set_type(web_sys::OscillatorType::Sine);
        high_osc.start_with_when(time)?;
        high_osc.stop_with_when(time + duration)?;

        let high_freq = high_osc.frequency();
        low_freq.set_value(Note::C3.freq());
        high_freq.exponential_ramp_to_value_at_time(Note::C4.freq(), time + attack)?;
        high_freq
            .exponential_ramp_to_value_at_time(Note::C3.freq(), time + attack + decay * 2.0)?;

        let amp_env = AmpEnvelope::new(self.ctx.clone(), volume, attack, decay, 0.0, 0.0);
        let low_amp = amp_env.node(&low_osc, time, duration)?;
        let high_amp = amp_env.node(&high_osc, time, duration)?;

        let osc_gain = self.ctx.create_gain()?;
        low_amp.connect_with_audio_node(&osc_gain)?;
        high_amp.connect_with_audio_node(&osc_gain)?;

        let osc_filter = self.ctx.create_biquad_filter()?;
        osc_filter.set_type(web_sys::BiquadFilterType::Highpass);
        osc_filter.frequency().set_value(osc_cutoff);
        osc_gain.connect_with_audio_node(&osc_filter)?;

        let mut noise_gen = Noise::new(self.ctx.clone());
        let noise = noise_gen.node(duration)?;
        noise.start_with_when(time)?;

        let noise_filter = self.ctx.create_biquad_filter()?;
        noise_filter.set_type(web_sys::BiquadFilterType::Highpass);
        noise_filter.frequency().set_value(noise_cutoff);
        noise.connect_with_audio_node(&noise_filter)?;

        let noise_amp_env =
            AmpEnvelope::new(self.ctx.clone(), noise_volume, attack, decay, 0.0, 0.0);
        let noise_amp = noise_amp_env.node(&noise_filter, time, duration)?;

        let output = self.ctx.create_gain()?;
        osc_filter.connect_with_audio_node(&output)?;
        noise_amp.connect_with_audio_node(&output)?;

        Ok(output)
    }
}
