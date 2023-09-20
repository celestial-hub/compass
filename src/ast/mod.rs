pub type Location = std::ops::Range<usize>;

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
  // x := y
  Assignment {
    destination: String,
    source: Operand,
    location: Location,
  },

  // x := y op z
  BinaryThenAssignment {
    destination: String,
    operation: BinaryOperation,
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
  Literal(i64),
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
