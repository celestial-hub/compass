use crate::lexer::{tokens::Token, types::Spanned, Lexer, LexicalError};

impl<'input> Iterator for Lexer<'input> {
  type Item = Spanned<Token, usize, LexicalError>;

  fn next(&mut self) -> Option<Self::Item> {
    self
      .token_stream
      .next()
      .map(|(tok, span)| Ok((span.start, tok.expect("Invalid token"), span.end)))
  }
}
