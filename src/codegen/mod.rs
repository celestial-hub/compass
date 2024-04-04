use crate::ast::Statement;
#[allow(warnings)] // TODO: remove me later
pub mod mips;

pub trait Codegen {
  fn generate(&self, ast: Vec<Statement>) -> Result<String, String>;
}
