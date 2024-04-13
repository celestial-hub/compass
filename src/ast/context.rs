// A hash map of the variables by scope level in the parser.

use std::collections::HashMap;

use crate::ast::{Argument, VarType};

use super::{Function, Statement, Variable};

// A struct to hold the context of the parser.
#[derive(Debug)]
pub struct Context {
  // The current scope level.
  pub scope_level: usize,
  // A hash map of the variables by scope level in the parser.
  pub variables: HashMap<usize, Vec<Variable>>,

  /// Function definitions
  pub functions: HashMap<String, Function>,

  pub optimization_level: u8,
}

impl Context {
  pub fn new(optimization_level: u8) -> Self {
    Self {
      scope_level: 0,
      optimization_level,
      variables: HashMap::new(),
      functions: HashMap::new(),
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

  pub fn add_function(&mut self, function: &Function) -> Result<(), String> {
    if self.functions.contains_key(&function.name) {
      return Err(format!("Function `{}` already defined", function.name));
    }

    self
      .functions
      .insert(function.name.clone(), function.clone());

    Ok(())
  }

  pub fn get_function(&self, name: &str) -> Option<Function> {
    self
      .builtins()
      .iter()
      .find(|f| f.name == name)
      .cloned()
      .or_else(|| self.functions.get(name).cloned())
  }

  fn builtins(&self) -> Vec<Function> {
    vec![
      Function {
        name: "write_string".to_string(),
        body: vec![],
        args: vec![Argument {
          name: "message".to_string(),
          var_type: VarType::Str,
        }],
        return_type: VarType::Void,
        location: 0..0,
        is_builtin: true,
      },
      Function {
        name: "write_int".to_string(),
        body: vec![],
        args: vec![Argument {
          name: "number".to_string(),
          var_type: VarType::I32,
        }],
        return_type: VarType::Void,
        location: 0..0,
        is_builtin: true,
      },
      Function {
        name: "read_int".to_string(),
        body: vec![],
        args: vec![],
        return_type: VarType::I32,
        location: 0..0,
        is_builtin: true,
      },
      Function {
        name: "read_string".to_string(),
        body: vec![],
        args: vec![Argument {
          name: "size".to_string(),
          var_type: VarType::U32,
        }],
        return_type: VarType::Str,
        location: 0..0,
        is_builtin: true,
      },
    ]
  }
}

impl Default for Context {
  fn default() -> Self {
    Self::new(0)
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
