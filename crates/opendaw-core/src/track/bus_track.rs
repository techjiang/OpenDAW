use crate::project::{TrackConfig, TrackId};

#[derive(Debug, Clone)]
pub struct BusTrack {
    id: TrackId,
    config: TrackConfig,
}

impl BusTrack {
    pub fn new(id: TrackId, config: TrackConfig) -> Self { Self { id, config } }
    pub fn id(&self) -> TrackId { self.id }
    pub fn name(&self) -> &str { &self.config.name }
}
