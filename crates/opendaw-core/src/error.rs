use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum OpenDawError {
    InvalidSampleRate(u32),
    InvalidBufferSize(u32),
    InvalidTempo(f64),
    InvalidTimeSignature(u32, u32),
    InvalidDuration,
    SplitOutOfRange { position: Duration, duration: Duration },
    StretchFactorOutOfRange(f64),
    TrackNotFound(u64),
    Io(String),
}

impl std::fmt::Display for OpenDawError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSampleRate(value) => write!(formatter, "invalid sample rate: {value}"),
            Self::InvalidBufferSize(value) => write!(formatter, "invalid buffer size: {value}"),
            Self::InvalidTempo(value) => write!(formatter, "invalid tempo: {value}"),
            Self::InvalidTimeSignature(a, b) => write!(formatter, "invalid time signature: {a}/{b}"),
            Self::InvalidDuration => write!(formatter, "invalid duration"),
            Self::SplitOutOfRange { position, duration } => {
                write!(formatter, "split position {:?} is outside clip duration {:?}", position, duration)
            }
            Self::StretchFactorOutOfRange(value) => write!(formatter, "invalid stretch or pitch factor: {value}"),
            Self::TrackNotFound(id) => write!(formatter, "track not found: {id}"),
            Self::Io(message) => write!(formatter, "I/O error: {message}"),
        }
    }
}

impl std::error::Error for OpenDawError {}

pub type Result<T> = std::result::Result<T, OpenDawError>;
