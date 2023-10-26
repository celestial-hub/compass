// A hash map of the variables by scope level in the parser.

use std::collections::HashMap;

use super::{Location, Operand, Statement, VarType};

// A struct to hold the context of the parser.
#[derive(Debug)]
pub struct Context {
  // The current scope level.
  pub scope_level: usize,
  // A hash map of the variables by scope level in the parser.
  pub variables: HashMap<usize, Vec<VariableInfo>>,
}

#[derive(Debug)]
pub struct VariableInfo {
  pub var_type: VarType,
  pub name: String,
  pub location: Location,
  pub value: Operand,
}

impl From<Statement> for VariableInfo {
  fn from(statement: Statement) -> Self {
    match statement {
      Statement::Variable {
        var_type,
        name,
        location,
        value,
      } => Self {
        var_type,
        name,
        location,
        value,
      },
      _ => panic!("Cannot convert statement to variable info"),
    }
  }
}

impl From<&VariableInfo> for Statement {
  fn from(variable_info: &VariableInfo) -> Self {
    Self::Variable {
      var_type: variable_info.var_type.clone(),
      name: variable_info.name.clone(),
      location: variable_info.location.clone(),
      value: variable_info.value.clone(),
    }
  }
}

impl Context {
  pub fn new() -> Self {
    Self {
      scope_level: 0,
      variables: HashMap::new(),
    }
  }

  pub fn push_scope(&mut self) {
    self.scope_level += 1;
  }

  pub fn pop_scope(&mut self) {
    self.scope_level -= 1;
  }

  pub fn add_variable(&mut self, statement: Statement) {
    let variable_info: VariableInfo = statement.into();

    if let Some(variables) = self.variables.get_mut(&self.scope_level) {
      variables.push(variable_info);
    } else {
      self.variables.insert(self.scope_level, vec![variable_info]);
    }
  }

  pub fn get_variable(&self, name: String) -> Option<&VariableInfo> {
    for scope in (0..=self.scope_level).rev() {
      if let Some(variables) = self.variables.get(&scope) {
        for variable in variables {
          if variable.name == name {
            return Some(variable);
          }
        }
      }
    }

    None
  }
}

impl Default for Context {
  fn default() -> Self {
    Self::new()
  }
}
