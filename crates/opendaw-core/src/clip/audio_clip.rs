use std::path::PathBuf;
use std::time::Duration;

use crate::error::{OpenDawError, Result};

use super::{Fade, FadeType};

#[derive(Debug, Clone, PartialEq)]
pub enum AudioSource {
    File(PathBuf),
    Generated { frames: usize, channels: usize },
}

#[derive(Debug, Clone, PartialEq)]
pub struct AudioClip {
    source: AudioSource,
    start: Duration,
    end: Duration,
    offset: Duration,
    fades: Vec<Fade>,
    gain: f32,
    pitch: f32,
    stretch_factor: f64,
}

impl AudioClip {
    pub fn from_file(source: PathBuf, start: Duration, end: Duration, _sample_rate: u32) -> Result<Self> {
        if end <= start { return Err(OpenDawError::InvalidDuration); }
        Ok(Self {
            source: AudioSource::File(source),
            start,
            end,
            offset: Duration::ZERO,
            fades: Vec::new(),
            gain: 1.0,
            pitch: 0.0,
            stretch_factor: 1.0,
        })
    }

    pub fn generated(duration: Duration, sample_rate: u32) -> Result<Self> {
        if duration.is_zero() || sample_rate == 0 { return Err(OpenDawError::InvalidDuration); }
        Ok(Self {
            source: AudioSource::Generated { frames: duration_to_frames(duration, sample_rate), channels: 2 },
            start: Duration::ZERO,
            end: duration,
            offset: Duration::ZERO,
            fades: Vec::new(),
            gain: 1.0,
            pitch: 0.0,
            stretch_factor: 1.0,
        })
    }

    pub fn source(&self) -> &AudioSource { &self.source }
    pub fn start(&self) -> Duration { self.start }
    pub fn end(&self) -> Duration { self.end }
    pub fn duration(&self) -> Duration { self.end.saturating_sub(self.start) }
    pub fn offset(&self) -> Duration { self.offset }
    pub fn gain(&self) -> f32 { self.gain }
    pub fn pitch(&self) -> f32 { self.pitch }
    pub fn stretch_factor(&self) -> f64 { self.stretch_factor }
    pub fn fades(&self) -> &[Fade] { &self.fades }

    pub fn set_gain(&mut self, gain: f32) { self.gain = gain; }
    pub fn set_offset(&mut self, offset: Duration) { self.offset = offset; }

    pub fn split(&self, position: Duration) -> Result<(AudioClip, AudioClip)> {
        if position <= self.start || position >= self.end {
            return Err(OpenDawError::SplitOutOfRange { position, duration: self.duration() });
        }
        let mut left = self.clone();
        let mut right = self.clone();
        left.end = position;
        right.start = position;
        right.offset = self.offset + (position - self.start);
        left.fades.retain(|fade| fade.kind == FadeType::In);
        right.fades.retain(|fade| fade.kind == FadeType::Out);
        Ok((left, right))
    }

    pub fn trim(&mut self, start: Duration, end: Duration) -> Result<()> {
        if end <= start || start < self.start || end > self.end { return Err(OpenDawError::InvalidDuration); }
        self.offset += start - self.start;
        self.start = start;
        self.end = end;
        Ok(())
    }

    pub fn set_fade(&mut self, kind: FadeType, duration: Duration) -> Result<()> {
        if duration > self.duration() { return Err(OpenDawError::InvalidDuration); }
        self.fades.retain(|fade| fade.kind != kind);
        self.fades.push(Fade { kind, duration });
        Ok(())
    }

    pub fn time_stretch(&mut self, factor: f64) -> Result<()> {
        if !factor.is_finite() || factor <= 0.0 { return Err(OpenDawError::StretchFactorOutOfRange(factor)); }
        self.stretch_factor = factor;
        let scaled = Duration::from_secs_f64(self.duration().as_secs_f64() * factor);
        self.end = self.start + scaled;
        Ok(())
    }

    pub fn pitch_shift(&mut self, semitones: f32) -> Result<()> {
        if !semitones.is_finite() || semitones.abs() > 60.0 { return Err(OpenDawError::StretchFactorOutOfRange(semitones as f64)); }
        self.pitch = semitones;
        Ok(())
    }
}

fn duration_to_frames(duration: Duration, sample_rate: u32) -> usize {
    (duration.as_secs_f64() * sample_rate as f64).round() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clip() -> AudioClip { AudioClip::generated(Duration::from_secs(4), 48_000).unwrap() }

    #[test]
    fn split_keeps_timeline_and_source_offset() {
        let clip = clip();
        let (left, right) = clip.split(Duration::from_secs(1)).unwrap();
        assert_eq!(left.duration(), Duration::from_secs(1));
        assert_eq!(right.duration(), Duration::from_secs(3));
        assert_eq!(right.offset(), Duration::from_secs(1));
    }

    #[test]
    fn trim_updates_source_offset() {
        let mut clip = clip();
        clip.trim(Duration::from_secs(1), Duration::from_secs(3)).unwrap();
        assert_eq!(clip.duration(), Duration::from_secs(2));
        assert_eq!(clip.offset(), Duration::from_secs(1));
    }

    #[test]
    fn stretch_and_pitch_are_non_destructive() {
        let mut clip = clip();
        clip.set_gain(0.8);
        clip.time_stretch(2.0).unwrap();
        clip.pitch_shift(3.0).unwrap();
        assert_eq!(clip.duration(), Duration::from_secs(8));
        assert_eq!(clip.gain(), 0.8);
        assert_eq!(clip.pitch(), 3.0);
        assert_eq!(clip.source(), &AudioSource::Generated { frames: 192_000, channels: 2 });
    }

    #[test]
    fn invalid_operations_rejected() {
        let mut clip = clip();
        assert!(clip.split(Duration::from_secs(9)).is_err());
        assert!(clip.split(Duration::ZERO).is_err());
        assert!(clip.time_stretch(0.0).is_err());
        assert!(clip.pitch_shift(61.0).is_err());
    }
}
