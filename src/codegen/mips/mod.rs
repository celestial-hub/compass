use crate::ast::Statement;

use super::Codegen;

pub struct MipsCodegen;

impl Codegen for MipsCodegen {
  fn generate(&self, ast: Vec<Statement>) -> String {
    let mut data_section = celestial_hub_astrolabe::ast::DataSection::default();
    let mut text_section = celestial_hub_astrolabe::ast::TextSection {
      entrypoint: "main".to_string(),
      ..Default::default()
    };

    let mut register_counter = 0;
    let mut register_map = std::collections::HashMap::new();

    for statement in ast {
      match statement {
        Statement::VariableDeclaration(var) => {
          let register = format!("${}", register_counter);
          register_counter += 1;
          register_map.insert(var.name.clone(), register.clone());

          match var.value {
            crate::ast::Expr::Operand(op) => match op {
              crate::ast::Operand::LiteralI8(val) => {
                text_section
                  .statements
                  .push(celestial_hub_astrolabe::ast::Statement::Instruction(
                    celestial_hub_astrolabe::ast::Instruction::Li(
                      [
                        celestial_hub_astrolabe::ast::InstructionArgument::Register(
                          celestial_hub_astrolabe::ast::Register { name: register },
                        ),
                        celestial_hub_astrolabe::ast::InstructionArgument::Immediate(val as u32),
                      ]
                      .into(),
                    ),
                  ));
              }
              crate::ast::Operand::LiteralStr(_) => todo!(),
              crate::ast::Operand::LiteralBool(_) => todo!(),
              crate::ast::Operand::LiteralI16(_) => todo!(),
              crate::ast::Operand::LiteralI32(_) => todo!(),
              crate::ast::Operand::LiteralI64(_) => todo!(),
              crate::ast::Operand::LiteralU8(_) => todo!(),
              crate::ast::Operand::LiteralU16(_) => todo!(),
              crate::ast::Operand::LiteralU32(_) => todo!(),
              crate::ast::Operand::LiteralU64(_) => todo!(),
              crate::ast::Operand::LiteralF32(_) => todo!(),
              crate::ast::Operand::LiteralF64(_) => todo!(),
              crate::ast::Operand::Identifier(_) => todo!(),
            },
            crate::ast::Expr::BinaryOperation(bin_op) => match bin_op {
              crate::ast::BinaryOperation::Arithmetic {
                lhs, operator, rhs, ..
              } => {
                if let (
                  crate::ast::Operand::Identifier(lhs),
                  crate::ast::Operand::Identifier(rhs),
                ) = (lhs, rhs)
                {
                  let lhs_register = register_map.get(&lhs).unwrap().clone();
                  let rhs_register = register_map.get(&rhs).unwrap().clone();

                  // Depending on the operator, we need to use different instructions
                  text_section.statements.push(
                    celestial_hub_astrolabe::ast::Statement::Instruction(match operator {
                      crate::ast::Operator::Add => celestial_hub_astrolabe::ast::Instruction::Add(
                        [
                          celestial_hub_astrolabe::ast::InstructionArgument::Register(
                            celestial_hub_astrolabe::ast::Register { name: register },
                          ),
                          celestial_hub_astrolabe::ast::InstructionArgument::Register(
                            celestial_hub_astrolabe::ast::Register { name: lhs_register },
                          ),
                          celestial_hub_astrolabe::ast::InstructionArgument::Register(
                            celestial_hub_astrolabe::ast::Register { name: rhs_register },
                          ),
                        ]
                        .into(),
                      ),
                      crate::ast::Operator::Sub => todo!(),
                      crate::ast::Operator::Mul => todo!(),
                      crate::ast::Operator::Div => todo!(),
                    }),
                  );
                }
              }
              crate::ast::BinaryOperation::Conditional {
                lhs,
                condition,
                rhs,
                operation_type,
              } => todo!(),
            },
            crate::ast::Expr::FunctionCall(_) => todo!(),
          }
        }
        Statement::ConditionalJump {
          condition,
          label,
          location,
        } => todo!(),
        Statement::UnconditionalJump { label, location } => todo!(),
        Statement::Label { name, location } => todo!(),
        Statement::FunctionDefinition(_) => todo!(),
      }
    }

    let program = celestial_hub_astrolabe::ast::Program {
      data_section,
      text_section,
    };

    format!("{program}")
  }
}
