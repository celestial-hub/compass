use compass::{lexer::Lexer, parser::Parser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let source_code = std::fs::read_to_string("assets/fibonacci.tac")?;
  let lexer = Lexer::new(&source_code[..]);
  let ast = Parser::new().parse(lexer)?;
  println!("{:#?}", ast);

  Ok(())
}
