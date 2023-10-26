use clap::Parser;
use compass::cli::{Cli, Commands};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let cli = Cli::parse();

  let _ = match &cli.command {
    Commands::Emit(options) => compass::cli::emit::ast(options),
  };

  Ok(())
}
