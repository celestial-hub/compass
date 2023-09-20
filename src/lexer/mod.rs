use logos::{Logos, SpannedIter};

use self::tokens::Token;

pub mod tokens;
pub mod traits;
pub mod types;

pub struct Lexer<'input> {
  pub token_stream: SpannedIter<'input, Token>,
}

#[derive(Debug)]
pub enum LexicalError {
  InvalidToken,
}

impl<'input> Lexer<'input> {
  pub fn new(input: &'input str) -> Self {
    let lexer = Self {
      token_stream: Token::lexer(input).spanned(),
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

    for (token, span) in token_stream.spanned() {
      if token.is_err() {
        use ariadne::{Color, ColorGenerator, Config, Fmt, Label, Report, ReportKind, Source};

        let mut colors = ColorGenerator::default();

        let a = colors.next();

        Report::build(ReportKind::Error, "sample.tac", 12)
          .with_code(3)
          .with_config(Config::default().with_tab_width(2))
          .with_message("Invalid token".fg(Color::Red))
          .with_label(
            Label::new(("comparison.tac", span))
              .with_message("Invalid token")
              .with_color(a),
          )
          .with_help("You probably added a character that is not allowed in the language")
          .with_note(format!(
            "If you think this is a bug, please file an issue at {}",
            "github.com/celestial-hub/compass/issues".fg(Color::Blue)
          ))
          .finish()
          .print((
            "comparison.tac",
            Source::from(include_str!("../../assets/fibonacci.tac")),
          ))
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
