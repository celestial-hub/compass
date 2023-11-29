use std::{ops::Range, str::FromStr};

use lalrpop_util::ParseError;

use crate::lexer::{tokens, ErrorTip, LexicalError};

pub type Location = std::ops::Range<usize>;
pub mod context;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VarType {
  I8,
  I16,
  I32,
  I64,
  U8,
  U16,
  U32,
  U64,
  Bool,
  F32,
  F64,
  Str,
  Void,
}

impl std::fmt::Display for VarType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      VarType::I8 => write!(f, "i8"),
      VarType::I16 => write!(f, "i16"),
      VarType::I32 => write!(f, "i32"),
      VarType::I64 => write!(f, "i64"),
      VarType::U8 => write!(f, "u8"),
      VarType::U16 => write!(f, "u16"),
      VarType::U32 => write!(f, "u32"),
      VarType::U64 => write!(f, "u64"),
      VarType::Bool => write!(f, "bool"),
      VarType::F32 => write!(f, "f32"),
      VarType::F64 => write!(f, "f64"),
      VarType::Str => write!(f, "str"),
      VarType::Void => write!(f, "void"),
    }
  }
}

impl FromStr for VarType {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "i8" => Ok(VarType::I8),
      "i16" => Ok(VarType::I16),
      "i32" => Ok(VarType::I32),
      "i64" => Ok(VarType::I64),
      "u8" => Ok(VarType::U8),
      "u16" => Ok(VarType::U16),
      "u32" => Ok(VarType::U32),
      "u64" => Ok(VarType::U64),
      "bool" => Ok(VarType::Bool),
      "f32" => Ok(VarType::F32),
      "f64" => Ok(VarType::F64),
      "str" => Ok(VarType::Str),
      _ => Err(format!("Invalid type: {}", s)),
    }
  }
}

impl From<String> for VarType {
  fn from(s: String) -> Self {
    s.as_str().parse().unwrap()
  }
}

impl From<VarType> for String {
  fn from(value: VarType) -> Self {
    match value {
      VarType::I8 => "i8".to_string(),
      VarType::I16 => "i16".to_string(),
      VarType::I32 => "i32".to_string(),
      VarType::I64 => "i64".to_string(),
      VarType::U8 => "u8".to_string(),
      VarType::U16 => "u16".to_string(),
      VarType::U32 => "u32".to_string(),
      VarType::U64 => "u64".to_string(),
      VarType::Bool => "bool".to_string(),
      VarType::F32 => "f32".to_string(),
      VarType::F64 => "f64".to_string(),
      VarType::Str => "str".to_string(),
      VarType::Void => "void".to_string(),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
  VariableDeclaration(Variable),

  // if x op y goto L
  ConditionalJump {
    condition: Expr,
    label: String,
    location: Location,
  },

  // goto L
  UnconditionalJump {
    label: String,
    location: Location,
  },

  // Label definition
  Label {
    name: String,
    location: Location,
  },

  FunctionDefinition(Function),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
  pub var_type: VarType,
  pub name: String,
  pub value: Expr,
  pub location: Location,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
  pub name: String,
  pub location: Location,
  pub args: Vec<Argument>,
  pub body: Vec<Statement>,
  pub return_type: VarType,
}

// Expressions are statements that return a value, such as
// function calls, arithmetic operations, literals, etc.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
  BinaryOperation(BinaryOperation),
  FunctionCall(Function),
  Operand(Operand),
}

// TODO: Impl get_type for Expr

#[derive(Clone, Debug, PartialEq)]
pub struct Return {
  pub var_type: VarType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Argument {
  pub name: String,
  pub var_type: VarType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperation {
  Arithmetic {
    lhs: Operand,
    operator: Operator,
    rhs: Operand,
    operation_type: VarType,
  },
  Conditional {
    lhs: Operand,
    condition: Condition,
    rhs: Operand,
    operation_type: VarType,
  },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
  Identifier(String),
  LiteralStr(String),
  LiteralBool(bool),
  LiteralI8(i8),
  LiteralI16(i16),
  LiteralI32(i32),
  LiteralI64(i64),
  LiteralU8(u8),
  LiteralU16(u16),
  LiteralU32(u32),
  LiteralU64(u64),
  LiteralF32(f32),
  LiteralF64(f64),
}

impl Operand {
  pub fn get_type(
    &self,
    context: &context::Context,
    loc: Range<usize>,
  ) -> Result<VarType, ParseError<usize, tokens::Token, LexicalError>> {
    match self {
      Operand::Identifier(variable_name) => {
        let var = context
          .get_variable(variable_name.clone())
          .ok_or(ParseError::<usize, tokens::Token, LexicalError>::User {
            error: LexicalError::UnknownVariable {
              error: vec![ErrorTip {
                message: format!("unknown variable `{}`", variable_name),
                location: loc,
              }],
              help: None,
            },
          })?;

        Ok(var.var_type)
      }
      Operand::LiteralStr(_) => Ok(VarType::Str),
      Operand::LiteralU8(_) => Ok(VarType::U8),
      Operand::LiteralU16(_) => Ok(VarType::U16),
      Operand::LiteralU32(_) => Ok(VarType::U32),
      Operand::LiteralU64(_) => Ok(VarType::U64),
      Operand::LiteralI8(_) => Ok(VarType::I8),
      Operand::LiteralI16(_) => Ok(VarType::I16),
      Operand::LiteralI32(_) => Ok(VarType::I32),
      Operand::LiteralI64(_) => Ok(VarType::I64),
      Operand::LiteralBool(_) => Ok(VarType::Bool),
      Operand::LiteralF32(_) => Ok(VarType::F32),
      Operand::LiteralF64(_) => Ok(VarType::F64),
    }
  }
}

impl TryFrom<Operand> for VarType {
  type Error = String;

  fn try_from(operand: Operand) -> Result<Self, Self::Error> {
    match operand {
      Operand::Identifier(_) => Err("Cannot convert identifier to type".to_string()),
      Operand::LiteralStr(_) => Ok(VarType::Str),
      Operand::LiteralU8(_) => Ok(VarType::U8),
      Operand::LiteralU16(_) => Ok(VarType::U16),
      Operand::LiteralU32(_) => Ok(VarType::U32),
      Operand::LiteralU64(_) => Ok(VarType::U64),
      Operand::LiteralI8(_) => Ok(VarType::I8),
      Operand::LiteralI16(_) => Ok(VarType::I16),
      Operand::LiteralI32(_) => Ok(VarType::I32),
      Operand::LiteralI64(_) => Ok(VarType::I64),
      Operand::LiteralBool(_) => Ok(VarType::Bool),
      Operand::LiteralF32(_) => Ok(VarType::F32),
      Operand::LiteralF64(_) => Ok(VarType::F64),
    }
  }
}

impl std::fmt::Display for Operand {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Operand::Identifier(name) => write!(f, "{}", name),
      Operand::LiteralStr(value) => write!(f, "{}", value),
      Operand::LiteralBool(value) => write!(f, "{}", value),
      Operand::LiteralI8(value) => write!(f, "{}", value),
      Operand::LiteralI16(value) => write!(f, "{}", value),
      Operand::LiteralI32(value) => write!(f, "{}", value),
      Operand::LiteralI64(value) => write!(f, "{}", value),
      Operand::LiteralU8(value) => write!(f, "{}", value),
      Operand::LiteralU16(value) => write!(f, "{}", value),
      Operand::LiteralU32(value) => write!(f, "{}", value),
      Operand::LiteralU64(value) => write!(f, "{}", value),
      Operand::LiteralF32(value) => write!(f, "{}", value),
      Operand::LiteralF64(value) => write!(f, "{}", value),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
  Add,
  Sub,
  Mul,
  Div,
}

impl std::fmt::Display for Operator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Operator::Add => write!(f, "+"),
      Operator::Sub => write!(f, "-"),
      Operator::Mul => write!(f, "*"),
      Operator::Div => write!(f, "/"),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Condition {
  LessThan,
  GreaterThan,
  LessThanOrEqual,
  GreaterThanOrEqual,
  Equal,
  NotEqual,
  And,
  Or,
}
