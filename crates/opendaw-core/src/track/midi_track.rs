use crate::project::{TrackConfig, TrackId};

#[derive(Debug, Clone)]
pub struct MidiTrack {
    id: TrackId,
    config: TrackConfig,
}

impl MidiTrack {
    pub fn new(id: TrackId, config: TrackConfig) -> Self { Self { id, config } }
    pub fn id(&self) -> TrackId { self.id }
    pub fn name(&self) -> &str { &self.config.name }
}
