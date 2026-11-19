use crate::dsp::AudioBuffer;

pub trait AudioEffect: Send {
    fn name(&self) -> &str;
    fn process(&mut self, buffer: &mut AudioBuffer);
    fn set_param(&mut self, id: &str, value: f32);
    fn get_param(&self, id: &str) -> f32;
    fn reset(&mut self);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChainMode { #[default] Serial, Parallel }

#[derive(Default)]
pub struct EffectChain {
    effects: Vec<Box<dyn AudioEffect>>,
    mode: ChainMode,
    bypassed: bool,
}

impl EffectChain {
    pub fn new() -> Self { Self::default() }
    pub fn with_mode(mode: ChainMode) -> Self { Self { mode, ..Self::default() } }
    pub fn add(&mut self, effect: Box<dyn AudioEffect>) { self.effects.push(effect); }
    pub fn len(&self) -> usize { self.effects.len() }
    pub fn is_empty(&self) -> bool { self.effects.is_empty() }
    pub fn mode(&self) -> ChainMode { self.mode }
    pub fn set_mode(&mut self, mode: ChainMode) { self.mode = mode; }
    pub fn set_bypassed(&mut self, bypassed: bool) { self.bypassed = bypassed; }
    pub fn is_bypassed(&self) -> bool { self.bypassed }

    pub fn process(&mut self, buffer: &mut AudioBuffer) {
        if self.bypassed || self.effects.is_empty() { return; }
        match self.mode {
            ChainMode::Serial => {
                for effect in &mut self.effects { effect.process(buffer); }
            }
            ChainMode::Parallel => {
                let mut mixed = buffer.clone();
                mixed.clear();
                for effect in &mut self.effects {
                    let mut branch = buffer.clone();
                    effect.process(&mut branch);
                    mixed.mix_from(&branch, 1.0 / self.effects.len() as f32);
                }
                *buffer = mixed;
            }
        }
    }

    pub fn reset(&mut self) {
        for effect in &mut self.effects { effect.reset(); }
    }

    pub fn set_param(&mut self, index: usize, id: &str, value: f32) -> bool {
        if let Some(effect) = self.effects.get_mut(index) { effect.set_param(id, value); true } else { false }
    }

    pub fn get_param(&self, index: usize, id: &str) -> Option<f32> {
        self.effects.get(index).map(|effect| effect.get_param(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Gain(f32);
    impl AudioEffect for Gain {
        fn name(&self) -> &str { "gain" }
        fn process(&mut self, buffer: &mut AudioBuffer) { buffer.apply_gain(self.0); }
        fn set_param(&mut self, _: &str, value: f32) { self.0 = value; }
        fn get_param(&self, _: &str) -> f32 { self.0 }
        fn reset(&mut self) { self.0 = 1.0; }
    }

    fn buffer(value: f32) -> AudioBuffer {
        let mut b = AudioBuffer::new(1, 2, 48_000);
        b.channel_mut(0).fill(value);
        b
    }

    #[test]
    fn serial_chain_applies_all_effects() {
        let mut chain = EffectChain::new();
        chain.add(Box::new(Gain(2.0)));
        chain.add(Box::new(Gain(0.5)));
        let mut b = buffer(1.0);
        chain.process(&mut b);
        assert_eq!(b.channel(0), &[1.0, 1.0]);
    }

    #[test]
    fn bypass_keeps_buffer_unchanged() {
        let mut chain = EffectChain::new();
        chain.add(Box::new(Gain(0.0)));
        chain.set_bypassed(true);
        let mut b = buffer(1.0);
        chain.process(&mut b);
        assert_eq!(b.channel(0), &[1.0, 1.0]);
    }
}
