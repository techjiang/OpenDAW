use std::time::Duration;

use crate::clip::AudioClip;
use crate::dsp::AudioBuffer;
use crate::project::{TrackConfig, TrackId, TrackKind};

#[derive(Debug, Clone)]
pub struct Track {
    id: TrackId,
    config: TrackConfig,
    clips: Vec<AudioClip>,
}

impl Track {
    pub fn new(id: TrackId, config: TrackConfig) -> Self {
        Self { id, config, clips: Vec::new() }
    }

    pub fn id(&self) -> TrackId { self.id }
    pub fn name(&self) -> &str { &self.config.name }
    pub fn kind(&self) -> TrackKind { self.config.kind }
    pub fn channels(&self) -> usize { self.config.channels.max(1) }
    pub fn config(&self) -> &TrackConfig { &self.config }
    pub fn clips(&self) -> &[AudioClip] { &self.clips }
    pub fn add_clip(&mut self, clip: AudioClip) { self.clips.push(clip); }

    pub fn remove_clip(&mut self, index: usize) -> Option<AudioClip> {
        if index < self.clips.len() { Some(self.clips.remove(index)) } else { None }
    }

    pub fn render(&self, buffer: &mut AudioBuffer, position: Duration, sample_rate: u32) {
        let start = position.as_secs_f64();
        let frame_duration = 1.0 / sample_rate as f64;
        for clip in &self.clips {
            let clip_start = clip.start().as_secs_f64();
            let clip_end = clip.end().as_secs_f64();
            if clip_end <= start { continue; }
            for frame in 0..buffer.frames() {
                let time = start + frame as f64 * frame_duration;
                if time < clip_start || time >= clip_end { continue; }
                let fade_gain = clip_fade_gain(clip, time - clip_start, clip_end - clip_start);
                let value = sample_for_clip(clip, time - clip_start) * clip.gain() * fade_gain;
                for channel in 0..buffer.channels().min(2) {
                    buffer.channel_mut(channel)[frame] += value;
                }
            }
        }
    }
}

fn sample_for_clip(clip: &AudioClip, local_time: f64) -> f32 {
    let phase = local_time * 2.0 * std::f32::consts::PI * 220.0 * 2f32.powf(clip.pitch() / 12.0) as f64;
    (phase.sin() * 0.25) as f32
}

fn clip_fade_gain(clip: &AudioClip, local_time: f64, duration: f64) -> f32 {
    let mut gain = 1.0;
    for fade in clip.fades() {
        let fade_duration = fade.duration.as_secs_f64();
        if fade_duration <= 0.0 { continue; }
        match fade.kind {
            crate::clip::FadeType::In => {
                if local_time < fade_duration { gain *= (local_time / fade_duration).clamp(0.0, 1.0) as f32; }
            }
            crate::clip::FadeType::Out => {
                let remaining = duration - local_time;
                if remaining < fade_duration { gain *= (remaining / fade_duration).clamp(0.0, 1.0) as f32; }
            }
        }
    }
    gain
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_applies_clip_gain_and_fade() {
        let config = TrackConfig::audio("test");
        let mut track = Track::new(TrackId(1), config);
        let mut clip = AudioClip::generated(Duration::from_secs(1), 48_000).unwrap();
        clip.set_gain(0.5);
        clip.set_fade(crate::clip::FadeType::In, Duration::from_millis(20)).unwrap();
        track.add_clip(clip);
        let mut buffer = AudioBuffer::new(2, 128, 48_000);
        track.render(&mut buffer, Duration::ZERO, 48_000);
        assert!(buffer.channel(0).iter().any(|sample| *sample != 0.0));
        assert_eq!(buffer.channel(0)[0], 0.0);
    }
}
