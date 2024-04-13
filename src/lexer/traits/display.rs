use crate::lexer::{self, tokens::Token, Lexer, LexicalError};

impl std::fmt::Display for LexicalError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      LexicalError::InvalidToken => write!(f, "Invalid token"),
      LexicalError::WrongType { error, help } => {
        for lexer::ErrorTip { message, location } in error {
          writeln!(f, "error: {message:?} at {location:?}")?;
        }
        match help {
          Some(help) => writeln!(f, "help: {help:?}"),
          None => Ok(()),
        }
      }
      LexicalError::UnknownVariable { error, help } => {
        for lexer::ErrorTip { message, location } in error {
          writeln!(f, "error: {message:?} at {location:?}")?;
        }
        match help {
          Some(help) => writeln!(f, "help: {help:?}"),
          None => Ok(()),
        }
      }
      LexicalError::UnknownFunction { error, help } => {
        for lexer::ErrorTip { message, location } in error {
          writeln!(f, "error: {message:?} at {location:?}")?;
        }
        match help {
          Some(help) => writeln!(f, "help: {help:?}"),
          None => Ok(()),
        }
      }
      LexicalError::WrongArgumentCount { error, help } => {
        for lexer::ErrorTip { message, location } in error {
          writeln!(f, "error: {message:?} at {location:?}")?;
        }
        match help {
          Some(help) => writeln!(f, "help: {help:?}"),
          None => Ok(()),
        }
      }
      LexicalError::FunctionIsBuiltin { error, help } => {
        for lexer::ErrorTip { message, location } in error {
          writeln!(f, "error: {message:?} at {location:?}")?;
        }
        match help {
          Some(help) => writeln!(f, "help: {help:?}"),
          None => Ok(()),
        }
      }
      LexicalError::UnusedValue { error, help } => {
        for lexer::ErrorTip { message, location } in error {
          writeln!(f, "error: {message:?} at {location:?}")?;
        }
        match help {
          Some(help) => writeln!(f, "help: {help:?}"),
          None => Ok(()),
        }
      }
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
