use crate::{lexer::Lexer, parser::Parser};

pub fn ast_from_code_str(code: &str, test_name: &str) -> String {
  let ast = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
    let lexer = Lexer::new(code, test_name).expect("Lexer to not fail in tests");
    Parser::new().parse(lexer)
  }));

  match ast.unwrap() {
    Ok(ast) => format!("{:#?}", ast),
    Err(err) => format!("{:#?}", err),
  }
}
