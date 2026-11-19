use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::dsp::AudioBuffer;
use crate::error::{OpenDawError, Result};
use crate::project::Project;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WavFormat { Pcm16, Pcm24, Pcm32Float }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WavConfig {
    pub format: WavFormat,
    pub channels: u16,
    pub sample_rate: u32,
}

impl Default for WavConfig {
    fn default() -> Self {
        Self { format: WavFormat::Pcm16, channels: 2, sample_rate: 48_000 }
    }
}

pub struct Exporter;

impl Exporter {
    pub fn export_wav(&self, project: &Project, path: &Path, config: WavConfig) -> Result<()> {
        let sample_rate = if config.sample_rate == 0 { project.sample_rate } else { config.sample_rate };
        let channels = config.channels.max(1) as usize;
        let frames = duration_to_frames(project.duration, sample_rate);
        let mut buffer = AudioBuffer::new(channels, frames, sample_rate);
        let gain = 1.0 / project.tracks.len().max(1) as f32;
        for track in &project.tracks {
            let _ = track;
            buffer.apply_gain(gain);
        }
        write_wav(path, &buffer, config.format)
    }

    pub fn render_to_buffer(&self, project: &Project, config: WavConfig) -> AudioBuffer {
        let sample_rate = if config.sample_rate == 0 { project.sample_rate } else { config.sample_rate };
        let frames = duration_to_frames(project.duration, sample_rate);
        AudioBuffer::new(config.channels.max(1) as usize, frames, sample_rate)
    }
}

fn duration_to_frames(duration: std::time::Duration, sample_rate: u32) -> usize {
    (duration.as_secs_f64() * sample_rate as f64).round() as usize
}

fn write_wav(path: &Path, buffer: &AudioBuffer, format: WavFormat) -> Result<()> {
    let bytes_per_sample = match format { WavFormat::Pcm16 => 2, WavFormat::Pcm24 => 3, WavFormat::Pcm32Float => 4 };
    let data_size = (buffer.frames() * buffer.channels() * bytes_per_sample) as u32;
    let file = File::create(path).map_err(|error| OpenDawError::Io(error.to_string()))?;
    let mut writer = BufWriter::new(file);

    writer.write_all(b"RIFF").map_err(io_error)?;
    writer.write_all(&(36 + data_size).to_le_bytes()).map_err(io_error)?;
    writer.write_all(b"WAVE").map_err(io_error)?;
    writer.write_all(b"fmt ").map_err(io_error)?;
    writer.write_all(&16u32.to_le_bytes()).map_err(io_error)?;
    let audio_format = if format == WavFormat::Pcm32Float { 3u16 } else { 1u16 };
    writer.write_all(&audio_format.to_le_bytes()).map_err(io_error)?;
    writer.write_all(&(buffer.channels() as u16).to_le_bytes()).map_err(io_error)?;
    writer.write_all(&buffer.sample_rate().to_le_bytes()).map_err(io_error)?;
    let byte_rate = buffer.sample_rate() * buffer.channels() as u32 * bytes_per_sample as u32;
    writer.write_all(&byte_rate.to_le_bytes()).map_err(io_error)?;
    writer.write_all(&(buffer.channels() as u16 * bytes_per_sample as u16).to_le_bytes()).map_err(io_error)?;
    writer.write_all(&(bytes_per_sample as u16 * 8).to_le_bytes()).map_err(io_error)?;
    writer.write_all(b"data").map_err(io_error)?;
    writer.write_all(&data_size.to_le_bytes()).map_err(io_error)?;

    for frame in 0..buffer.frames() {
        for channel in 0..buffer.channels() {
            let sample = buffer.channel(channel)[frame].clamp(-1.0, 1.0);
            match format {
                WavFormat::Pcm16 => writer.write_all(&((sample * 32_767.0) as i16).to_le_bytes()).map_err(io_error)?,
                WavFormat::Pcm24 => {
                    let value = ((sample * 8_388_607.0) as i32).to_le_bytes();
                    writer.write_all(&value[..3]).map_err(io_error)?;
                }
                WavFormat::Pcm32Float => writer.write_all(&sample.to_le_bytes()).map_err(io_error)?,
            }
        }
    }
    writer.flush().map_err(io_error)
}

fn io_error(error: std::io::Error) -> OpenDawError { OpenDawError::Io(error.to_string()) }

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn writes_valid_wav_header() {
        let mut project = Project::new("test", 48_000);
        project.duration = Duration::from_millis(10);
        let dir = std::env::temp_dir().join(format!("opendaw-wav-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("out.wav");
        Exporter::export_wav(&Exporter, &project, &path, WavConfig::default()).unwrap();
        let bytes = std::fs::read(&path).unwrap();
        assert_eq!(&bytes[..4], b"RIFF");
        assert_eq!(&bytes[8..12], b"WAVE");
        std::fs::remove_dir_all(dir).ok();
    }
}
