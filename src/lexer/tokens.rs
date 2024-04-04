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
  #[token("func")]
  Function,
  #[token("begin")]
  Begin,
  #[token("end")]
  End,
  #[token("return")]
  Return,

  // Types
  #[regex("str|i8|i16|i32|i64|u8|u16|u32|u64|f32|f64|bool", |lex| lex.slice().parse().ok())]
  Type(String),

  // Identifiers (variables, labels, etc.)
  #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().parse().ok())]
  Identifier(String),

  // Integer Literals
  #[regex("[+-]?[0-9]+", |lex| lex.slice().parse().ok())]
  #[regex("[+-]?[0-9]+i32", |lex| lex.slice().strip_suffix("i32").unwrap().parse().ok())]
  LiteralI32(i32),

  #[regex("[+-]?[0-9]+i8", |lex| lex.slice().strip_suffix("i8").unwrap().parse().ok())]
  LiteralI8(i8),

  #[regex("[+-]?[0-9]+i16", |lex| lex.slice().strip_suffix("i16").unwrap().parse().ok())]
  LiteralI16(i16),

  #[regex("[+-]?[0-9]+i64", |lex| lex.slice().strip_suffix("i64").unwrap().parse().ok())]
  LiteralI64(i64),

  #[regex("[0-9]+u8", |lex| lex.slice().strip_suffix("u8").unwrap().parse().ok())]
  LiteralU8(u8),

  #[regex("[0-9]+u16", |lex| lex.slice().strip_suffix("u16").unwrap().parse().ok())]
  LiteralU16(u16),

  #[regex("[0-9]+u32", |lex| lex.slice().strip_suffix("u32").unwrap().parse().ok())]
  LiteralU32(u32),

  #[regex("[0-9]+u64", |lex| lex.slice().strip_suffix("u64").unwrap().parse().ok())]
  LiteralU64(u64),

  // Float 32 Literals
  // Valids are 0.0, .0, 0., 0f32, 0.0f32, .0f32, 0.f32
  #[regex(r"[0-9]*\.[0-9]+(f32)?", |lex| {
    let slice = lex.slice().strip_suffix("f32");
    if slice.is_some() {
      slice.unwrap().parse().ok()
    } else {
      lex.slice().parse().ok()
    }
  })]
  #[regex(r"[0-9]+\.(f32)?", |lex| {
    let slice = lex.slice().strip_suffix("f32");
    if slice.is_some() {
      slice.unwrap().parse().ok()
    } else {
      lex.slice().parse().ok()
    }
  })]
  #[regex(r"[0-9]+f32", |lex| lex.slice().strip_suffix("f32").unwrap().parse().ok())]
  LiteralF32(f32),

  // Float 64 Literals
  #[regex(r"[0-9]*\.[0-9]+f64", |lex| {
    let slice = lex.slice().strip_suffix("f64");
    if slice.is_some() {
      slice.unwrap().parse().ok()
    } else {
      lex.slice().parse().ok()
    }
  })]
  #[regex(r"[0-9]+\.f64", |lex| {
    let slice = lex.slice().strip_suffix("f64");
    if slice.is_some() {
      slice.unwrap().parse().ok()
    } else {
      lex.slice().parse().ok()
    }
  })]
  #[regex(r"[0-9]+f64", |lex| lex.slice().strip_suffix("f64").unwrap().parse().ok())]
  LiteralF64(f64),

  // String literal
  #[regex("\"[^\"]*\"", |lex| lex.slice().parse().ok())]
  LiteralString(String),

  // Boolean literals
  #[token("true")]
  LiteralTrue,

  #[token("false")]
  LiteralFalse,

  // Punctuation
  #[token("[")]
  OpenBracket,
  #[token("]")]
  CloseBracket,
  #[token("(")]
  OpenParen,
  #[token(")")]
  CloseParen,
  #[token(":")]
  Colon,
}
