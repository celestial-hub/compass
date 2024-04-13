use ariadne::ReportBuilder;
use lalrpop_util::{lalrpop_mod, ParseError};

use crate::{
  ast::{self, context::Context},
  lexer::{Lexer, LexicalError},
};

pub struct Parser;

lalrpop_mod!(pub compass_grammar, "/parser/compass_grammar.rs");

impl Parser {
  pub fn new() -> Self {
    Self
  }

  pub fn parse(&self, lexer: Lexer) -> Result<Vec<ast::Statement>, Box<dyn std::error::Error>> {
    use ariadne::{Color, ColorGenerator, Config, Fmt, Label, Report, ReportKind, Source};

    let mut colors = ColorGenerator::default();

    let filename = lexer.filepath.split('/').last().unwrap();
    let source = lexer.source_code;

    let mut report: ReportBuilder<(&str, std::ops::Range<usize>)> =
      Report::build(ReportKind::Error, filename, 12)
        .with_code(3)
        .with_config(Config::default().with_tab_width(2))
        .with_note(format!(
          "If you think this is a bug, please file an issue at {}",
          "github.com/celestial-hub/compass/issues".fg(Color::Blue)
        ));

    // NOTE: I should probably use this context in the codegen module
    let mut context = Context::new(0);
    match compass_grammar::ProgramParser::new().parse(&mut context, lexer) {
      Ok(ast) => Ok(ast),
      Err(err) => match err {
        ParseError::InvalidToken { location } => {
          report
            .with_message("Invalid token".fg(Color::Red))
            .with_label(
              Label::new((filename, location..location))
                .with_message("Invalid token")
                .with_color(colors.next()),
            )
            .finish()
            .print((filename, Source::from(source)))
            .unwrap();

          Err(Box::new(err))
        }
        ParseError::User { ref error } => {
          let (errors, help) = match error {
            LexicalError::WrongType { error, help } => (error, help),
            LexicalError::UnknownVariable { error, help } => (error, help),
            LexicalError::UnknownFunction { error, help } => (error, help),
            LexicalError::WrongArgumentCount { error, help } => (error, help),
            LexicalError::FunctionIsBuiltin { error, help } => (error, help),
            LexicalError::UnusedValue { error, help } => (error, help),
            LexicalError::InvalidToken => unreachable!(),
          };

          report = report.with_message("Error".fg(Color::Red));

          for error in errors {
            report = report.with_label(
              Label::new((filename, error.location.clone()))
                .with_message(error.message.clone())
                .with_color(colors.next()),
            );
          }

          if let Some(help) = help {
            report = report.with_help(help.clone());
          }

          report
            .finish()
            .print((filename, Source::from(source)))
            .unwrap();

          Err(Box::new(err))
        }
        ParseError::UnrecognizedToken {
          ref token,
          ref expected,
        } => {
          report
            .with_message("Unrecognized token".fg(Color::Red))
            .with_label(
              Label::new((filename, token.0..token.2))
                .with_message("Unrecognized token")
                .with_color(colors.next()),
            )
            .with_help(format!(
              "Expected one of the following: {}",
              expected
                .iter()
                .map(|token| {
                  // Remove surrounding quotes
                  let token = &token[1..token.len() - 1];

                  format!("{}", token.fg(Color::Yellow))
                })
                .collect::<Vec<String>>()
                .join(", ")
            ))
            .finish()
            .print((filename, Source::from(source)))
            .unwrap();
          Err(Box::new(err))
        }
        _ => Err(Box::new(err)),
      },
    }
  }
}

impl Default for Parser {
  fn default() -> Self {
    Self::new()
  }
}
