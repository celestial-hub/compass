use std::collections::HashMap;

use celestial_hub_astrolabe::ast::{DataSection, TextSection};

use crate::ast::{Argument, Function, VarType};

// Actually most of the usefull information of context is already in the AST. So we could just
// return it and use.
pub struct Context {
  pub data_section: DataSection,
  pub text_section: TextSection,
  pub register_counter: u32,
  pub register_map: HashMap<String, String>,
  pub function_map: HashMap<String, Function>,
  pub scope_level: u32,
  pub conditional_counter: u32,
  pub buffer_counter: u32,
}

impl Context {
  pub fn new() -> Self {
    Self {
      data_section: DataSection::default(),
      text_section: TextSection::default(),
      register_counter: 0,
      scope_level: 0,
      conditional_counter: 0,
      buffer_counter: 0,
      register_map: HashMap::new(),
      function_map: HashMap::new(),
    }
  }

  pub fn get_register(&mut self, name: &str) -> String {
    if let Some(register) = self.register_map.get(name) {
      return register.clone();
    }

    let register = format!("${}", self.register_counter);
    self.register_counter += 1;

    self.register_map.insert(name.to_string(), register.clone());

    register
  }

  pub fn get_function(&self, name: &str) -> Option<Function> {
    self
      .builtins()
      .iter()
      .find(|f| f.name == name)
      .cloned()
      .or_else(|| self.function_map.get(&format!("__{name}")).cloned())
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
    Self::new()
  }
}
