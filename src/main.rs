use clap::Parser;
use compass::cli::{Cli, Commands};
use compass::codegen::Codegen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let cli = Cli::parse();

  let ast = match &cli.command {
    Commands::Emit(options) => compass::cli::emit::ast(options),
  }
  .map_err(|e| e.to_string())?;

  let mips_codegen = compass::codegen::mips::MipsCodegen;

  mips_codegen.generate(ast);

  Ok(())
}
