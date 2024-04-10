use celestial_hub_compass::cli::{Cli, Commands};
use celestial_hub_compass::codegen::Codegen;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let cli = Cli::parse();

  let ast = match &cli.command {
    Commands::Emit(options) => celestial_hub_compass::cli::emit::ast(options),
  }?;

  let mips_codegen = celestial_hub_compass::codegen::mips::MipsCodegen;

  let result = mips_codegen.generate(ast, &mut Default::default())?;

  println!("{}", result);

  Ok(())
}
