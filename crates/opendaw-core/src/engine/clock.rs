use std::time::Duration;
use crate::error::{OpenDawError, Result};

#[derive(Debug, Clone)]
pub struct Clock {
    sample_rate: u32,
    tempo: f64,
    numerator: u32,
    denominator: u32,
}

impl Clock {
    pub fn new(sample_rate: u32, tempo: f64, numerator: u32, denominator: u32) -> Result<Self> {
        if sample_rate == 0 { return Err(OpenDawError::InvalidSampleRate(sample_rate)); }
        if !tempo.is_finite() || tempo <= 0.0 { return Err(OpenDawError::InvalidTempo(tempo)); }
        if numerator == 0 || denominator == 0 { return Err(OpenDawError::InvalidTimeSignature(numerator, denominator)); }
        Ok(Self { sample_rate, tempo, numerator, denominator })
    }

    pub fn sample_rate(&self) -> u32 { self.sample_rate }
    pub fn tempo(&self) -> f64 { self.tempo }
    pub fn time_signature(&self) -> (u32, u32) { (self.numerator, self.denominator) }

    pub fn set_tempo(&mut self, tempo: f64) -> Result<()> {
        if !tempo.is_finite() || tempo <= 0.0 { return Err(OpenDawError::InvalidTempo(tempo)); }
        self.tempo = tempo;
        Ok(())
    }

    pub fn set_time_signature(&mut self, numerator: u32, denominator: u32) -> Result<()> {
        if numerator == 0 || denominator == 0 { return Err(OpenDawError::InvalidTimeSignature(numerator, denominator)); }
        self.numerator = numerator;
        self.denominator = denominator;
        Ok(())
    }

    pub fn samples_per_beat(&self) -> f64 {
        self.sample_rate as f64 * 60.0 / self.tempo
    }

    pub fn duration_to_samples(&self, duration: Duration) -> u64 {
        let seconds = duration.as_secs_f64();
        (seconds * self.sample_rate as f64).round() as u64
    }

    pub fn samples_to_duration(&self, samples: u64) -> Duration {
        Duration::from_secs_f64(samples as f64 / self.sample_rate as f64)
    }

    pub fn bars_and_beats(&self, samples: u64) -> (u64, f64) {
        let beats = samples as f64 / self.samples_per_beat();
        let beats_per_bar = self.numerator as f64 * 4.0 / self.denominator as f64;
        let bar = (beats / beats_per_bar).floor() as u64;
        let beat_in_bar = beats % beats_per_bar;
        (bar, beat_in_bar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tempo_conversion() {
        let clock = Clock::new(48_000, 120.0, 4, 4).unwrap();
        assert_eq!(clock.samples_per_beat(), 24_000.0);
        assert_eq!(clock.duration_to_samples(Duration::from_secs(1)), 48_000);
    }

    #[test]
    fn invalid_values_rejected() {
        assert!(Clock::new(0, 120.0, 4, 4).is_err());
        assert!(Clock::new(48_000, 0.0, 4, 4).is_err());
        assert!(Clock::new(48_000, 120.0, 0, 4).is_err());
    }
}
