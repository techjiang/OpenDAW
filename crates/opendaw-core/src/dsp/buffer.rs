#[derive(Debug, Clone, PartialEq)]
pub struct AudioBuffer {
    channels: Vec<Vec<f32>>,
    sample_rate: u32,
}

impl AudioBuffer {
    pub fn new(channels: usize, frames: usize, sample_rate: u32) -> Self {
        Self { channels: vec![vec![0.0; frames]; channels], sample_rate }
    }

    pub fn from_channels(channels: Vec<Vec<f32>>, sample_rate: u32) -> Self {
        let frames = channels.first().map_or(0, Vec::len);
        debug_assert!(channels.iter().all(|channel| channel.len() == frames));
        Self { channels, sample_rate }
    }

    pub fn channels(&self) -> usize { self.channels.len() }
    pub fn frames(&self) -> usize { self.channels.first().map_or(0, Vec::len) }
    pub fn sample_rate(&self) -> u32 { self.sample_rate }
    pub fn channel(&self, index: usize) -> &[f32] { &self.channels[index] }
    pub fn channel_mut(&mut self, index: usize) -> &mut [f32] { &mut self.channels[index] }
    pub fn iter_channels(&self) -> impl Iterator<Item = &Vec<f32>> { self.channels.iter() }
    pub fn iter_channels_mut(&mut self) -> impl Iterator<Item = &mut Vec<f32>> { self.channels.iter_mut() }

    pub fn clear(&mut self) {
        for channel in &mut self.channels { channel.fill(0.0); }
    }

    pub fn mix_from(&mut self, other: &AudioBuffer, gain: f32) {
        debug_assert_eq!(self.channels(), other.channels());
        debug_assert_eq!(self.frames(), other.frames());
        for (dst, src) in self.channels.iter_mut().zip(other.iter_channels()) {
            for (dst_sample, src_sample) in dst.iter_mut().zip(src) {
                *dst_sample += *src_sample * gain;
            }
        }
    }

    pub fn apply_gain(&mut self, gain: f32) {
        for channel in &mut self.channels {
            for sample in channel { *sample *= gain; }
        }
    }

    pub fn apply_fade(&mut self, curve: FadeCurve) {
        let frames = self.frames();
        if frames == 0 { return; }
        for channel in &mut self.channels {
            for (i, sample) in channel.iter_mut().enumerate() {
                let x = i as f32 / (frames - 1).max(1) as f32;
                let gain = match curve {
                    FadeCurve::LinearIn => x,
                    FadeCurve::LinearOut => 1.0 - x,
                    FadeCurve::EqualPowerIn => (x * std::f32::consts::FRAC_PI_2).sin(),
                    FadeCurve::EqualPowerOut => ((1.0 - x) * std::f32::consts::FRAC_PI_2).cos(),
                };
                *sample *= gain;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FadeCurve { LinearIn, LinearOut, EqualPowerIn, EqualPowerOut }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mix_and_gain_work() {
        let mut a = AudioBuffer::new(1, 4, 48_000);
        let mut b = AudioBuffer::new(1, 4, 48_000);
        a.channel_mut(0).fill(0.5);
        b.channel_mut(0).fill(0.25);
        a.mix_from(&b, 2.0);
        assert_eq!(a.channel(0), &[1.0, 1.0, 1.0, 1.0]);
        a.apply_gain(0.5);
        assert_eq!(a.channel(0), &[0.5, 0.5, 0.5, 0.5]);
    }

    #[test]
    fn fade_in_starts_at_zero() {
        let mut buffer = AudioBuffer::new(1, 4, 48_000);
        buffer.channel_mut(0).fill(1.0);
        buffer.apply_fade(FadeCurve::LinearIn);
        assert_eq!(buffer.channel(0)[0], 0.0);
        assert_eq!(buffer.channel(0)[3], 1.0);
    }
}
