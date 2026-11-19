use crate::dsp::{AudioBuffer, EffectChain};

#[derive(Default)]
pub struct Mixer {
    channels: Vec<(String, f32, f32, EffectChain)>,
}

impl Mixer {
    pub fn add_channel(&mut self, name: impl Into<String>, gain: f32, pan: f32) -> usize {
        self.channels.push((name.into(), gain, pan, EffectChain::new()));
        self.channels.len() - 1
    }

    pub fn channel_count(&self) -> usize { self.channels.len() }
    pub fn channel_name(&self, index: usize) -> &str { &self.channels[index].0 }
    pub fn gain(&self, index: usize) -> f32 { self.channels[index].1 }
    pub fn pan(&self, index: usize) -> f32 { self.channels[index].2 }
    pub fn set_gain(&mut self, index: usize, gain: f32) { self.channels[index].1 = gain; }
    pub fn set_pan(&mut self, index: usize, pan: f32) { self.channels[index].2 = pan.clamp(-1.0, 1.0); }
    pub fn effects_mut(&mut self, index: usize) -> &mut EffectChain { &mut self.channels[index].3 }

    pub fn mix(&mut self, inputs: &[AudioBuffer]) -> AudioBuffer {
        let channels = inputs.first().map_or(2, AudioBuffer::channels);
        let frames = inputs.first().map_or(0, AudioBuffer::frames);
        let sample_rate = inputs.first().map_or(48_000, AudioBuffer::sample_rate);
        let mut output = AudioBuffer::new(channels, frames, sample_rate);

        for (i, input) in inputs.iter().enumerate().take(self.channels.len()) {
            let (_, gain, pan, effects) = &mut self.channels[i];
            let mut processed = input.clone();
            effects.process(&mut processed);
            for channel_index in 0..processed.channels().min(channels) {
                let pan_gain = pan_gain_for(*pan, channel_index);
                let total_gain = *gain * pan_gain;
                for (dst, src) in output.channel_mut(channel_index).iter_mut().zip(processed.channel(channel_index)) {
                    *dst += *src * total_gain;
                }
            }
        }
        output
    }
}

fn pan_gain_for(pan: f32, channel_index: usize) -> f32 {
    if channel_index == 0 {
        (1.0 - pan.max(0.0)).min(1.0).max(0.0)
    } else if channel_index == 1 {
        (1.0 + pan.min(0.0)).max(0.0).min(1.0)
    } else {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixer_applies_gain() {
        let mut mixer = Mixer::default();
        mixer.add_channel("drums", 0.5, 0.0);
        let mut input = AudioBuffer::new(2, 2, 48_000);
        input.channel_mut(0).fill(1.0);
        input.channel_mut(1).fill(1.0);
        let output = mixer.mix(&[input]);
        assert_eq!(output.channel(0), &[0.5, 0.5]);
        assert_eq!(output.channel(1), &[0.5, 0.5]);
    }

    #[test]
    fn pan_controls_stereo_balance() {
        let mut mixer = Mixer::default();
        mixer.add_channel("lead", 1.0, 1.0);
        let mut input = AudioBuffer::new(2, 1, 48_000);
        input.channel_mut(0).fill(1.0);
        input.channel_mut(1).fill(1.0);
        let output = mixer.mix(&[input]);
        assert_eq!(output.channel(0)[0], 0.0);
        assert_eq!(output.channel(1)[0], 1.0);
    }
}
