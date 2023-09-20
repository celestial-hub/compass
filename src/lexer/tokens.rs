use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r"#.*")]
pub enum Token {
  // Operators
  #[token("+")]
  Add,
  #[token("-")]
  Sub,
  #[token("*")]
  Mul,
  #[token("/")]
  Div,
  #[token("=")]
  Assign,
  #[token("<")]
  LessThan,
  #[token(">")]
  GreaterThan,
  #[token("<=")]
  LessThanOrEqual,
  #[token(">=")]
  GreaterThanOrEqual,
  #[token("==")]
  Equal,
  #[token("!=")]
  NotEqual,
  #[token("&&")]
  And,
  #[token("||")]
  Or,

  // Keywords
  #[token("if")]
  If,
  #[token("goto")]
  Goto,

  // Identifiers (variables, labels, etc.)
  #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().parse().ok())]
  Identifier(String),

  // Literals
  #[regex("[0-9]+", |lex| lex.slice().parse().ok())]
  Literal(i64),

  // Punctuation
  #[token("[")]
  OpenBracket,
  #[token("]")]
  CloseBracket,
  #[token(":")]
  Colon,
}
