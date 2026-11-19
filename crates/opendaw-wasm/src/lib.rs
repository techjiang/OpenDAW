use opendaw_core::AudioEngine;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmEngine {
    engine: AudioEngine,
}

#[wasm_bindgen]
impl WasmEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(sample_rate: u32, buffer_size: u32) -> Result<WasmEngine, JsValue> {
        let engine = AudioEngine::new(sample_rate, buffer_size)
            .map_err(|error| JsValue::from_str(&error.to_string()))?;
        Ok(Self { engine })
    }

    pub fn play(&mut self) { self.engine.play(); }
    pub fn pause(&mut self) { self.engine.pause(); }
    pub fn stop(&mut self) { self.engine.stop(); }
    pub fn set_tempo(&mut self, bpm: f64) -> Result<(), JsValue> {
        self.engine.set_tempo(bpm).map_err(|error| JsValue::from_str(&error.to_string()))
    }
}
