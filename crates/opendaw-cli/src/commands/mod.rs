use std::path::PathBuf;

use opendaw_core::{save_project, AudioClip, AudioEngine, Exporter, Project, TrackConfig, WavConfig, WavFormat};

#[derive(Debug)]
pub enum Cli {
    Info,
    New { name: String, sample_rate: u32, output: Option<PathBuf> },
    Render { input: PathBuf, output: PathBuf, sample_rate: u32 },
    Help,
}

impl Cli {
    pub fn parse(args: impl IntoIterator<Item = String>) -> Self {
        let args: Vec<String> = args.into_iter().collect();
        match args.as_slice() {
            [command] if command == "info" => Self::Info,
            [command] if command == "help" || command == "--help" || command == "-h" => Self::Help,
            [command, name] if command == "new" => Self::New { name: name.clone(), sample_rate: 48_000, output: None },
            [command, name, sample_rate] if command == "new" => Self::New {
                name: name.clone(),
                sample_rate: sample_rate.parse().unwrap_or(48_000),
                output: None,
            },
            [command, name, sample_rate, output] if command == "new" => Self::New {
                name: name.clone(),
                sample_rate: sample_rate.parse().unwrap_or(48_000),
                output: Some(PathBuf::from(output)),
            },
            [command, input, output] if command == "render" => Self::Render {
                input: PathBuf::from(input),
                output: PathBuf::from(output),
                sample_rate: 48_000,
            },
            [command, input, output, sample_rate] if command == "render" => Self::Render {
                input: PathBuf::from(input),
                output: PathBuf::from(output),
                sample_rate: sample_rate.parse().unwrap_or(48_000),
            },
            _ => Self::Help,
        }
    }
}

pub fn run(cli: Cli) -> Result<(), String> {
    match cli {
        Cli::Info => {
            println!("OpenDAW v{}", env!("CARGO_PKG_VERSION"));
            println!("Core engine: Rust workspace with WAV export and project I/O");
            println!("Capabilities: multi-track engine, clips, effects chain, mixer, WAV export");
        }
        Cli::Help => print_help(),
        Cli::New { name, sample_rate, output } => {
            let mut engine = AudioEngine::new(sample_rate, 64).map_err(|error| error.to_string())?;
            let id = engine.add_track(TrackConfig::audio(&name));
            let clip = AudioClip::generated(std::time::Duration::from_secs(2), sample_rate)
                .map_err(|error| error.to_string())?;
            engine.add_clip(id, clip).map_err(|error| error.to_string())?;
            let mut project: Project = engine.project();
            project.name = name.clone();
            project.duration = std::time::Duration::from_secs(2);
            println!("created project {:?} with track {:?} at {} Hz", name, id, sample_rate);
            if let Some(path) = output {
                save_project(&project, &path).map_err(|error| error.to_string())?;
                println!("saved project to {}", path.display());
            }
        }
        Cli::Render { input, output, sample_rate } => {
            let project = opendaw_core::load_project(&input).map_err(|error| error.to_string())?;
            let config = WavConfig {
                format: WavFormat::Pcm16,
                channels: 2,
                sample_rate: if sample_rate == 0 { project.sample_rate } else { sample_rate },
            };
            Exporter.export_wav(&project, &output, config).map_err(|error| error.to_string())?;
            println!("rendered {} to {}", project.name, output.display());
        }
    }
    Ok(())
}

fn print_help() {
    println!("OpenDAW CLI");
    println!();
    println!("Usage:");
    println!("  opendaw-cli info");
    println!("  opendaw-cli new <project-name> [sample-rate] [output.json]");
    println!("  opendaw-cli render <project.json> <output.wav> [sample-rate]");
    println!("  opendaw-cli help");
}
