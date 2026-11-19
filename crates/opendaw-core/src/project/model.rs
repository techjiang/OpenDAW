use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TrackId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TrackKind { #[default] Audio, Midi, Bus }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AudioSource {
    File(PathBuf),
    Generated { frames: usize, channels: usize },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackConfig {
    pub name: String,
    pub kind: TrackKind,
    pub channels: usize,
}

impl TrackConfig {
    pub fn audio(name: impl Into<String>) -> Self {
        Self { name: name.into(), kind: TrackKind::Audio, channels: 2 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub sample_rate: u32,
    pub tracks: Vec<TrackConfig>,
    pub tempo: f64,
    pub time_signature: (u32, u32),
    pub duration: Duration,
    pub sources: HashMap<TrackId, AudioSource>,
}

impl Project {
    pub fn new(name: impl Into<String>, sample_rate: u32) -> Self {
        Self {
            name: name.into(),
            sample_rate,
            tracks: Vec::new(),
            tempo: 120.0,
            time_signature: (4, 4),
            duration: Duration::ZERO,
            sources: HashMap::new(),
        }
    }

    pub fn track_count(&self) -> usize { self.tracks.len() }
}
