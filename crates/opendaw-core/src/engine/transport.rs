use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TransportState { #[default] Stopped, Playing, Paused, Recording }

#[derive(Debug, Clone)]
pub struct Transport {
    state: TransportState,
    position: Duration,
    loop_start: Option<Duration>,
    loop_end: Option<Duration>,
}

impl Default for Transport {
    fn default() -> Self {
        Self { state: TransportState::Stopped, position: Duration::ZERO, loop_start: None, loop_end: None }
    }
}

impl Transport {
    pub fn state(&self) -> TransportState { self.state }
    pub fn position(&self) -> Duration { self.position }
    pub fn set_position(&mut self, position: Duration) { self.position = position; }

    pub fn play(&mut self) { self.state = TransportState::Playing; }
    pub fn pause(&mut self) { if self.state != TransportState::Stopped { self.state = TransportState::Paused; } }
    pub fn stop(&mut self) { self.state = TransportState::Stopped; self.position = Duration::ZERO; }
    pub fn record(&mut self) { self.state = TransportState::Recording; }

    pub fn set_loop(&mut self, start: Duration, end: Duration) {
        if start <= end {
            self.loop_start = Some(start);
            self.loop_end = Some(end);
        }
    }

    pub fn clear_loop(&mut self) { self.loop_start = None; self.loop_end = None; }

    pub fn advance(&mut self, duration: Duration) {
        if self.state == TransportState::Stopped { return; }
        self.position += duration;
        if let (Some(start), Some(end)) = (self.loop_start, self.loop_end) {
            if end > start && self.position >= end {
                let span = end - start;
                let overrun = (self.position - end).as_nanos() % span.as_nanos().max(1);
                self.position = start + Duration::from_nanos(overrun as u64);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stop_resets_position() {
        let mut t = Transport::default();
        t.play();
        t.advance(Duration::from_secs(2));
        t.stop();
        assert_eq!(t.state(), TransportState::Stopped);
        assert_eq!(t.position(), Duration::ZERO);
    }

    #[test]
    fn loop_wraps_position() {
        let mut t = Transport::default();
        t.set_loop(Duration::from_secs(1), Duration::from_secs(3));
        t.play();
        t.set_position(Duration::from_secs(2));
        t.advance(Duration::from_secs(2));
        assert_eq!(t.position(), Duration::from_secs(1));
    }
}
