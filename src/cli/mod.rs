pub mod emit;
pub mod eval;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(propagate_version = true)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  Emit(emit::EmitASTOptions),
}
