use std::time::Duration;

use opendaw_core::clip::{AudioClip, FadeType};
use opendaw_core::{AudioEngine, Exporter, Project, TrackConfig, WavConfig};

#[test]
fn m1_workflow_smoke() {
    let mut engine = AudioEngine::new(48_000, 64).unwrap();
    let track = engine.add_track(TrackConfig::audio("Lead"));
    let mut clip = AudioClip::generated(Duration::from_secs(2), 48_000).unwrap();
    clip.set_fade(FadeType::In, Duration::from_millis(10)).unwrap();
    clip.set_fade(FadeType::Out, Duration::from_millis(10)).unwrap();
    engine.add_clip(track, clip).unwrap();
    engine.play();
    assert_eq!(engine.process_block().frames(), 64);

    let mut project: Project = engine.project();
    project.duration = Duration::from_millis(20);
    let path = std::env::temp_dir().join(format!("opendaw-m1-{}.wav", std::process::id()));
    Exporter.export_wav(&project, &path, WavConfig::default()).unwrap();
    let bytes = std::fs::read(&path).unwrap();
    assert_eq!(&bytes[..4], b"RIFF");
    std::fs::remove_file(path).ok();
}
