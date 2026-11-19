use opendaw_core::{AudioEngine, TrackConfig};

#[derive(Debug)]
pub enum Cli {
    Info,
    New { name: String, sample_rate: u32 },
}

impl Cli {
    pub fn parse(args: impl IntoIterator<Item = String>) -> Self {
        let args: Vec<String> = args.into_iter().collect();
        match args.as_slice() {
            [command] if command == "info" => Self::Info,
            [command, name, sample_rate] if command == "new" => Self::New {
                name: name.clone(),
                sample_rate: sample_rate.parse().unwrap_or(48_000),
            },
            _ => Self::Info,
        }
    }
}

pub fn run(cli: Cli) -> Result<(), String> {
    match cli {
        Cli::Info => {
            println!("OpenDAW v{}", env!("CARGO_PKG_VERSION"));
            println!("Core engine: Rust + C++ plugin host");
        }
        Cli::New { name, sample_rate } => {
            let mut engine = AudioEngine::new(sample_rate, 64).map_err(|error| error.to_string())?;
            let id = engine.add_track(TrackConfig::audio(&name));
            println!("created track {:?} at {} Hz", id, sample_rate);
        }
    }
    Ok(())
}
