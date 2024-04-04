use celestial_hub_compass::cli::{Cli, Commands};
use celestial_hub_compass::codegen::Codegen;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let cli = Cli::parse();

  let ast = match &cli.command {
    Commands::Emit(options) => celestial_hub_compass::cli::emit::ast(options),
  }
  .map_err(|e| e.to_string())?;

  let mips_codegen = celestial_hub_compass::codegen::mips::MipsCodegen;

  let result = mips_codegen.generate(ast)?;

  println!("{}", result);

  Ok(())
}
