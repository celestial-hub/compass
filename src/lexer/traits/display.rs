use crate::lexer::{tokens::Token, Lexer, LexicalError};

impl std::fmt::Display for LexicalError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      LexicalError::InvalidToken => write!(f, "Invalid token"),
    }
  }
}

impl<'input> std::fmt::Display for Lexer<'input> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    for (token, span) in self.token_stream.clone().spanned() {
      let token = token.unwrap();
      writeln!(f, "{{ {:?} {:?} }}", token, span)?;
    }

    Ok(())
  }
}

impl std::fmt::Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}
