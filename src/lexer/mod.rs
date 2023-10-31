use logos::{Logos, SpannedIter};

use self::tokens::Token;

pub mod tokens;
pub mod traits;
pub mod types;

pub struct Lexer<'input> {
  pub token_stream: SpannedIter<'input, Token>,
  pub filepath: &'input str,
  pub source_code: &'input str,
}

#[derive(Debug)]
pub struct ErrorTip {
  pub message: String,
  pub location: std::ops::Range<usize>,
}

#[derive(Debug)]
pub enum LexicalError {
  InvalidToken,
  WrongType {
    error: Vec<ErrorTip>,
    help: Option<String>,
  },
  UnknownVariable {
    error: Vec<ErrorTip>,
    help: Option<String>,
  },
}

impl<'input> Lexer<'input> {
  pub fn new(source_code: &'input str, filepath: &'input str) -> Self {
    let lexer = Self {
      token_stream: Token::lexer(source_code).spanned(),
      filepath,
      source_code,
    };

    if lexer.validate().is_err() {
      std::process::exit(1);
    }

    lexer
  }
}

impl<'input> Lexer<'input> {
  fn validate(&self) -> Result<(), LexicalError> {
    let token_stream = self.token_stream.clone();
    let mut error = false;

    let filename = self.filepath.split('/').last().unwrap();

    for (token, span) in token_stream.spanned() {
      if token.is_err() {
        use ariadne::{Color, ColorGenerator, Config, Fmt, Label, Report, ReportKind, Source};

        let mut colors = ColorGenerator::default();
        let color = colors.next();

        Report::build(ReportKind::Error, filename, 12)
          .with_code(3)
          .with_config(Config::default().with_tab_width(2))
          .with_message("Invalid token".fg(Color::Red))
          .with_label(
            Label::new((filename, span))
              .with_message("Invalid token")
              .with_color(color),
          )
          .with_help("You probably added a character that is not allowed in the language")
          .with_note(format!(
            "If you think this is a bug, please file an issue at {}",
            "github.com/celestial-hub/compass/issues".fg(Color::Blue)
          ))
          .finish()
          .print((filename, Source::from(self.source_code)))
          .unwrap();

        error = true;
      }
    }

    if error {
      return Err(LexicalError::InvalidToken);
    }

    Ok(())
  }
}
