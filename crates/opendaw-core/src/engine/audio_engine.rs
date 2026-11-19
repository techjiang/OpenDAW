use std::time::Duration;

use crate::clip::AudioClip;
use crate::dsp::AudioBuffer;
use crate::error::{OpenDawError, Result};
use crate::project::{AudioSource, Project, TrackConfig, TrackId};
use crate::track::Track;

use super::{Clock, Mixer, Transport, TransportState};

pub struct AudioEngine {
    sample_rate: u32,
    buffer_size: u32,
    tracks: Vec<Track>,
    next_track_id: u64,
    mixer: Mixer,
    transport: Transport,
    clock: Clock,
}

impl AudioEngine {
    pub fn new(sample_rate: u32, buffer_size: u32) -> Result<Self> {
        if sample_rate == 0 { return Err(OpenDawError::InvalidSampleRate(sample_rate)); }
        if buffer_size == 0 { return Err(OpenDawError::InvalidBufferSize(buffer_size)); }
        Ok(Self {
            sample_rate,
            buffer_size,
            tracks: Vec::new(),
            next_track_id: 1,
            mixer: Mixer::default(),
            transport: Transport::default(),
            clock: Clock::new(sample_rate, 120.0, 4, 4)?,
        })
    }

    pub fn sample_rate(&self) -> u32 { self.sample_rate }
    pub fn buffer_size(&self) -> u32 { self.buffer_size }
    pub fn tracks(&self) -> &[Track] { &self.tracks }
    pub fn transport(&self) -> &Transport { &self.transport }
    pub fn transport_mut(&mut self) -> &mut Transport { &mut self.transport }
    pub fn clock(&self) -> &Clock { &self.clock }
    pub fn mixer_mut(&mut self) -> &mut Mixer { &mut self.mixer }

    pub fn add_track(&mut self, config: TrackConfig) -> TrackId {
        let id = TrackId(self.next_track_id);
        self.next_track_id += 1;
        self.mixer.add_channel(&config.name, 1.0, 0.0);
        self.tracks.push(Track::new(id, config));
        id
    }

    pub fn remove_track(&mut self, id: TrackId) -> Result<()> {
        let index = self.tracks.iter().position(|track| track.id() == id)
            .ok_or(OpenDawError::TrackNotFound(id.0))?;
        self.tracks.remove(index);
        Ok(())
    }

    pub fn add_clip(&mut self, track_id: TrackId, clip: AudioClip) -> Result<()> {
        let track = self.tracks.iter_mut().find(|track| track.id() == track_id)
            .ok_or(OpenDawError::TrackNotFound(track_id.0))?;
        track.add_clip(clip);
        Ok(())
    }

    pub fn play(&mut self) { self.transport.play(); }
    pub fn pause(&mut self) { self.transport.pause(); }
    pub fn stop(&mut self) { self.transport.stop(); }
    pub fn record(&mut self) { self.transport.record(); }

    pub fn set_tempo(&mut self, bpm: f64) -> Result<()> { self.clock.set_tempo(bpm) }
    pub fn set_time_signature(&mut self, num: u32, den: u32) -> Result<()> { self.clock.set_time_signature(num, den) }

    pub fn process_block(&mut self) -> AudioBuffer {
        let frames = self.buffer_size as usize;
        let channels = self.tracks.first().map_or(2, |track| track.channels());
        let mut inputs = Vec::with_capacity(self.tracks.len());
        for track in &self.tracks {
            let mut buffer = AudioBuffer::new(channels, frames, self.sample_rate);
            if self.transport.state() != TransportState::Stopped {
                track.render(&mut buffer, self.transport.position(), self.sample_rate);
            }
            inputs.push(buffer);
        }
        self.transport.advance(Duration::from_secs_f64(frames as f64 / self.sample_rate as f64));
        self.mixer.mix(&inputs)
    }

    pub fn project(&self) -> Project {
        let mut project = Project::new("Untitled", self.sample_rate);
        project.tempo = self.clock.tempo();
        project.time_signature = self.clock.time_signature();
        project.tracks = self.tracks.iter().map(|track| track.config().clone()).collect();
        for track in &self.tracks {
            if let Some(clip) = track.clips().first() {
                project.sources.insert(track.id(), clip.source().clone());
            }
        }
        project
    }

    pub fn set_project(&mut self, project: &Project) -> Result<()> {
        self.clock = Clock::new(project.sample_rate, project.tempo, project.time_signature.0, project.time_signature.1)?;
        self.tracks.clear();
        self.next_track_id = 1;
        self.mixer = Mixer::default();
        for config in &project.tracks {
            let id = self.add_track(config.clone());
            if let Some(source) = project.sources.get(&id) {
                if let AudioSource::File(path) = source {
                    let clip = AudioClip::from_file(path.clone(), Duration::ZERO, Duration::from_secs(1), self.sample_rate)?;
                    self.add_clip(id, clip)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_lifecycle() {
        let mut engine = AudioEngine::new(48_000, 64).unwrap();
        let id = engine.add_track(TrackConfig::audio("Audio 1"));
        assert!(engine.add_clip(id, AudioClip::generated(Duration::from_secs(1), 48_000)).is_ok());
        engine.play();
        let output = engine.process_block();
        assert_eq!(output.frames(), 64);
        assert!(engine.transport().position() > Duration::ZERO);
        engine.stop();
        assert_eq!(engine.transport().position(), Duration::ZERO);
    }

    #[test]
    fn invalid_config_rejected() {
        assert!(AudioEngine::new(0, 64).is_err());
        assert!(AudioEngine::new(48_000, 0).is_err());
    }
}
