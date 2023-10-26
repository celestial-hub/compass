use std::str::FromStr;

pub type Location = std::ops::Range<usize>;
pub mod context;

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
  Variable {
    var_type: VarType,
    name: String,
    value: Operand,
    location: Location,
  },

  // if x op y goto L
  ConditionalJump {
    lhs: Operand,
    condition: Condition,
    rhs: Operand,
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

  // Function definition
  Function {
    name: String,
    location: Location,
    args: Vec<Argument>,
    body: Vec<Statement>,
    return_type: VarType,
  },
}

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
  },
  Conditional {
    lhs: Operand,
    condition: Condition,
    rhs: Operand,
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

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
  Add,
  Sub,
  Mul,
  Div,
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
