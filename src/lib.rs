use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys::Uint8Array, AudioBuffer, AudioBufferSourceNode, AudioContext};

async fn load_sample(sample: &[u8]) -> Result<AudioBuffer, JsValue> {
    let ctx = AudioContext::new()?;
    let array_buffer = Uint8Array::from(sample).buffer();
    let decoded = JsFuture::from(ctx.decode_audio_data(&array_buffer)?).await?;
    Ok(AudioBuffer::from(decoded))
}

#[wasm_bindgen]
pub struct Player {
    ctx: AudioContext,
    src: AudioBufferSourceNode,
}

impl Drop for Player {
    fn drop(&mut self) {
        let _ = self.ctx.close();
    }
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(constructor)]
    pub async fn new() -> Result<Player, JsValue> {
        let ctx = AudioContext::new()?;
        let sample = include_bytes!("../samples/a3.wav");
        let buffer = load_sample(sample).await?;

        let src = ctx.create_buffer_source()?;
        src.set_buffer(Some(&buffer));
        src.connect_with_audio_node(&ctx.destination())?;

        Ok(Self { ctx, src })
    }

    pub fn start(&self) -> Result<(), JsValue> {
        self.src.start()?;
        Ok(())
    }

    pub fn stop(&self) -> Result<(), JsValue> {
        self.src.stop()?;
        Ok(())
    }
}
