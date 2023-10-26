use ariadne::ReportBuilder;
use lalrpop_util::{lalrpop_mod, ParseError};

use crate::{
  ast::{self, context::Context},
  lexer::Lexer,
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

    let a = colors.next();

    let filename = lexer.filepath.split('/').last().unwrap();
    let filepath = lexer.filepath;
    let source = std::fs::read_to_string(filepath).unwrap();

    let report: ReportBuilder<(&str, std::ops::Range<usize>)> =
      Report::build(ReportKind::Error, filename, 12)
        .with_code(3)
        .with_config(Config::default().with_tab_width(2))
        .with_note(format!(
          "If you think this is a bug, please file an issue at {}",
          "github.com/celestial-hub/compass/issues".fg(Color::Blue)
        ));

    match compass_grammar::ProgramParser::new().parse(&mut Context::default(), lexer) {
      Ok(ast) => Ok(ast),
      Err(err) => match err {
        ParseError::InvalidToken { location } => {
          report
            .with_message("Invalid token".fg(Color::Red))
            .with_label(
              Label::new((filename, location..location))
                .with_message("Invalid token")
                .with_color(a),
            )
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
                .with_color(a),
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
