use celestial_hub_astrolabe::{
  ast::{
    DataSection, Instruction, InstructionArgument, Program, Register, Statement, TextSection,
    Variable,
  },
  lexer::tokens::{Type, Value},
};

use crate::ast::{BinaryOperation, Expr, Operand, Operator, Statement as CompassStatement};
use std::collections::HashMap;

use super::Codegen;

pub struct MipsCodegen;

macro_rules! create_instruction {
  ($instruction:path, $register:expr, $lhs_register:expr, $rhs:expr) => {
    Statement::Instruction($instruction(
      [
        InstructionArgument::Register(Register { name: $register }),
        InstructionArgument::Register(Register {
          name: $lhs_register,
        }),
        $rhs,
      ]
      .into(),
    ))
  };
}

impl Codegen for MipsCodegen {
  fn generate(&self, ast: Vec<CompassStatement>) -> Result<String, String> {
    let mut data_section = DataSection::default();
    let mut text_section = TextSection {
      entrypoint: "main".to_string(),
      ..Default::default()
    };

    let mut register_counter = 0;
    let mut register_map: std::collections::HashMap<String, String> =
      std::collections::HashMap::new();

    for statement in ast {
      match statement {
        CompassStatement::VariableDeclaration(var) => {
          let register = find_or_create_reg(&mut register_map, var.name.clone());

          match var.value {
            Expr::Operand(op) => match op {
              Operand::LiteralI8(val) => load_immediate(&mut text_section, register, val as u32),
              Operand::LiteralI16(val) => load_immediate(&mut text_section, register, val as u32),
              Operand::LiteralI32(val) => load_immediate(&mut text_section, register, val as u32),
              Operand::LiteralI64(val) => load_immediate(&mut text_section, register, val as u32),
              Operand::LiteralU8(val) => load_immediate(&mut text_section, register, val as u32),
              Operand::LiteralU16(val) => load_immediate(&mut text_section, register, val as u32),
              Operand::LiteralU32(val) => load_immediate(&mut text_section, register, val),
              Operand::LiteralU64(val) => {
                return Err("Cannot store a 64-bit integer in a 32-bit register".to_string());
              }
              Operand::LiteralStr(val) => {
                load_string(&mut text_section, &mut data_section, register, val)
              }
              Operand::LiteralBool(val) => todo!(),
              Operand::LiteralF32(val) => todo!(),
              Operand::LiteralF64(val) => todo!(),
              Operand::Identifier(var) => todo!(),
            },
            Expr::BinaryOperation(bin_op) => match bin_op {
              BinaryOperation::Arithmetic {
                lhs, operator, rhs, ..
              } => {
                if is_register(&lhs) && is_register(&rhs) {
                  let lhs = lhs.as_identifier()?;
                  let rhs = rhs.as_identifier()?;

                  let lhs_register = register_map
                    .get(lhs)
                    .ok_or_else(|| format!("Register {} not found", lhs))?
                    .clone();
                  let rhs_register = register_map
                    .get(rhs)
                    .ok_or_else(|| format!("Register {} not found", rhs))?
                    .clone();

                  text_section.statements.push(match operator {
                    Operator::Add => create_instruction!(
                      Instruction::Add,
                      register,
                      lhs_register,
                      InstructionArgument::Register(Register { name: rhs_register })
                    ),
                    Operator::Sub => create_instruction!(
                      Instruction::Sub,
                      register,
                      lhs_register,
                      InstructionArgument::Register(Register { name: rhs_register })
                    ),
                    Operator::Mul => todo!(),
                    Operator::Div => todo!(),
                  });
                } else if is_register(&lhs) && is_immediate(&rhs) {
                  let lhs = lhs.as_identifier()?;
                  let lhs_register = register_map
                    .get(lhs)
                    .ok_or_else(|| format!("Register {} not found", lhs))?
                    .clone();
                  let rhs_value = rhs.as_immediate()?;

                  text_section.statements.push(match operator {
                    Operator::Add => create_instruction!(
                      Instruction::Addi,
                      register,
                      lhs_register,
                      InstructionArgument::Immediate(rhs_value)
                    ),
                    // Handle other operators (Sub, Mul, Div) similarly...
                    _ => todo!(),
                  });
                } else if is_immediate(&lhs) && is_immediate(&rhs) {
                  let rhs = rhs.as_immediate()?;
                  let lhs_value = lhs.as_immediate()?;

                  // There is no instruction that can add two immediates, so we need to load one of them into a register first
                  let lhs_register = new_register(&mut register_map);
                  load_immediate(&mut text_section, lhs_register.clone(), lhs_value);

                  text_section.statements.push(match operator {
                    Operator::Add => create_instruction!(
                      Instruction::Addi,
                      register,
                      lhs_register,
                      InstructionArgument::Immediate(rhs)
                    ),
                    // Handle other operators (Sub, Mul, Div) similarly...
                    _ => todo!(),
                  });
                } else {
                  Err(format!(
                    "Invalid operands for arithmetic operation {} and {}",
                    lhs, rhs
                  ))?;
                }
                // Handle other cases...
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
        CompassStatement::ConditionalJump {
          condition,
          label,
          location,
        } => todo!(),
        CompassStatement::UnconditionalJump { label, location } => todo!(),
        CompassStatement::Label { name, location } => todo!(),
        CompassStatement::FunctionDefinition(_) => todo!(),
      }
    }

    let program = Program {
      data_section,
      text_section,
    };

    Ok(program.to_string())
  }
}

fn find_or_create_reg(register_map: &mut HashMap<String, String>, name: String) -> String {
  if let Some(register) = register_map.get(&name) {
    register.clone()
  } else {
    let register = format!("${}", register_map.len());
    register_map.insert(name.clone(), register.clone());
    register
  }
}

fn new_register(register_map: &mut HashMap<String, String>) -> String {
  let register = format!("${}", register_map.len());
  register_map.insert(register.clone(), register.clone());
  register
}

fn is_register(value: &crate::ast::Operand) -> bool {
  match value {
    Operand::Identifier(_) => true,
    _ => false,
  }
}

fn is_immediate(value: &crate::ast::Operand) -> bool {
  match value {
    Operand::LiteralI8(_) => true,
    Operand::LiteralI16(_) => true,
    Operand::LiteralI32(_) => true,
    Operand::LiteralI64(_) => true,
    Operand::LiteralU8(_) => true,
    Operand::LiteralU16(_) => true,
    Operand::LiteralU32(_) => true,
    Operand::LiteralU64(_) => true,
    _ => false,
  }
}

fn load_immediate(text_section: &mut TextSection, register: String, value: u32) {
  text_section
    .statements
    .push(Statement::Instruction(Instruction::Li(
      [
        InstructionArgument::Register(Register { name: register }),
        InstructionArgument::Immediate(value),
      ]
      .into(),
    )));
}

fn load_string(
  text_section: &mut TextSection,
  data_section: &mut DataSection,
  register: String,
  value: String,
) {
  // Search for an existing string label
  let mut label = String::new();
  let mut found = false;
  for (i, variable) in data_section.variables.iter().enumerate() {
    if let Value::String(existing_value) = &variable.value {
      if existing_value == &value {
        label = format!("str_{}", i);
        found = true;
        break;
      }
    }
  }

  // If the string is not found, add it to the data section
  if !found {
    label = format!("str_{}", data_section.variables.len());
    data_section.variables.push(Variable {
      name: label.clone(),
      type_: Type::Asciiz,
      value: Value::String(value),
    });
  }

  // Load the address of the string into the register
  text_section
    .statements
    .push(Statement::Instruction(Instruction::La(
      [
        InstructionArgument::Register(Register { name: register }),
        InstructionArgument::Label(label),
      ]
      .into(),
    )));
}
