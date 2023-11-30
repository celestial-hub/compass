use crate::ast::Statement;

pub mod mips;

pub trait Codegen {
  fn generate(&self, ast: Vec<Statement>);
}
