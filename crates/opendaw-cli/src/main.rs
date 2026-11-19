mod commands;

use commands::{run, Cli};

fn main() {
    let cli = Cli::parse(std::env::args().skip(1));
    if let Err(error) = run(cli) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
