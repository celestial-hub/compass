use clap::Args;

use crate::{ast::Statement, lexer::Lexer, parser::Parser};

#[derive(Args)]
pub struct EmitASTOptions {
  /// The ETAC file to parse
  #[arg(short = 'f')]
  pub filepath: String,

  /// Turn debugging information on
  #[arg(short, long, action = clap::ArgAction::Count)]
  debug: u8,
}

pub fn ast(
  EmitASTOptions { filepath, debug }: &EmitASTOptions,
) -> Result<Vec<Statement>, Box<dyn std::error::Error>> {
  let source_code = std::fs::read_to_string(filepath)?;
  let lexer = Lexer::new(&source_code[..], filepath);

  if *debug > 0 {
    println!("{}", lexer);
  }

  let ast = Parser::new().parse(lexer)?;

  if *debug > 0 {
    println!("{:#?}", ast);
  }

  Ok(ast)
}
