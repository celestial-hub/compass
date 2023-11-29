// A hash map of the variables by scope level in the parser.

use std::collections::HashMap;

use super::{Statement, Variable};

// A struct to hold the context of the parser.
#[derive(Debug)]
pub struct Context {
  // The current scope level.
  pub scope_level: usize,
  // A hash map of the variables by scope level in the parser.
  pub variables: HashMap<usize, Vec<Variable>>,
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
    let variable = statement.into();

    if let Some(variables) = self.variables.get_mut(&self.scope_level) {
      variables.push(variable);
    } else {
      self.variables.insert(self.scope_level, vec![variable]);
    }
  }

  pub fn get_variable(&self, name: String) -> Option<&Variable> {
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

impl From<Statement> for Variable {
  fn from(statement: Statement) -> Self {
    match statement {
      Statement::VariableDeclaration(var) => var,
      _ => panic!("Cannot convert statement to variable"),
    }
  }
}
