use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use wasm_bindgen::JsValue;
use web_sys::{AudioBufferSourceNode, AudioContext};

pub struct Noise {
    ctx: AudioContext,
    rng: ChaCha8Rng,
}

#[allow(dead_code)]
impl Noise {
    pub fn new(ctx: AudioContext) -> Self {
        let rng = ChaCha8Rng::from_entropy();
        Self { ctx, rng }
    }

    pub fn node(&mut self, duration: f64) -> Result<AudioBufferSourceNode, JsValue> {
        let frames = (self.ctx.sample_rate() as f64 * duration).round() as u32;
        let buffer = self.ctx.create_buffer(1, frames, self.ctx.sample_rate())?;
        let mut data = buffer.get_channel_data(0)?;

        for _ in 0..frames {
            data[0] = self.rng.gen_range(-1.0..1.0);
        }

        let src = self.ctx.create_buffer_source()?;
        src.set_buffer(Some(&buffer));
        Ok(src)
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    use super::*;

    #[wasm_bindgen_test]
    fn test_node() {
        let ctx = AudioContext::new().unwrap();
        let mut noise = Noise::new(ctx);
        let node = noise.node(3.0).unwrap();
        let buffer = node.buffer().unwrap();
        assert_eq!(buffer.length(), 44100 * 3);
    }
}
