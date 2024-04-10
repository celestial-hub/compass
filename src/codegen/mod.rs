use crate::ast::Statement;

use self::context::Context;
pub(crate) mod context;
#[allow(warnings)] // TODO: remove me later
pub mod mips;

pub trait Codegen {
  fn generate(&self, ast: Vec<Statement>, context: &mut Context) -> Result<String, String>;
}
