use celestial_hub_astrolabe::{
  ast::{
    DataSection, Instruction, InstructionArgument, Program, Register, Statement, TextSection,
    Variable,
  },
  lexer::tokens::{Type, Value},
};

use crate::ast::{
  context, BinaryOperation, Condition, Expr, Function, Operand, Operator,
  Statement as CompassStatement, VarType,
};
use std::collections::HashMap;

use super::{context::Context, Codegen};

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
  fn generate(&self, ast: Vec<CompassStatement>, context: &mut Context) -> Result<String, String> {
    if context.scope_level == 0 {
      context
        .text_section
        .statements
        .push(Statement::Label("main".to_string()));
    }

    for statement in ast {
      match statement {
        CompassStatement::VariableDeclaration(var) => {
          let register = find_or_create_reg(&mut context.register_map, var.name.clone());

          match var.value {
            Expr::Operand(op) => match op {
              Operand::LiteralI8(val) => {
                load_immediate(&mut context.text_section, register, val as u32)
              }
              Operand::LiteralI16(val) => {
                load_immediate(&mut context.text_section, register, val as u32)
              }
              Operand::LiteralI32(val) => {
                load_immediate(&mut context.text_section, register, val as u32)
              }
              Operand::LiteralI64(val) => {
                load_immediate(&mut context.text_section, register, val as u32)
              }
              Operand::LiteralU8(val) => {
                load_immediate(&mut context.text_section, register, val as u32)
              }
              Operand::LiteralU16(val) => {
                load_immediate(&mut context.text_section, register, val as u32)
              }
              Operand::LiteralU32(val) => load_immediate(&mut context.text_section, register, val),
              Operand::LiteralStr(val) => load_string(
                &mut context.text_section,
                &mut context.data_section,
                register,
                val,
              ),
              Operand::LiteralU64(val) => {
                return Err("Cannot store a 64-bit integer in a 32-bit register".to_string());
              }
              Operand::LiteralBool(val) => todo!(),
              Operand::LiteralF32(val) => todo!(),
              Operand::LiteralF64(val) => todo!(),
              Operand::Identifier(var) => todo!(),
              Operand::Dereference(_) => todo!(),
            },
            Expr::BinaryOperation(bin_op) => match bin_op {
              BinaryOperation::Arithmetic {
                lhs, operator, rhs, ..
              } => {
                if is_register(&lhs) && is_register(&rhs) {
                  let lhs = lhs.as_identifier()?;
                  let rhs = rhs.as_identifier()?;

                  let lhs_register = context
                    .register_map
                    .get(lhs)
                    .ok_or_else(|| format!("Register {} not found", lhs))?
                    .clone();
                  let rhs_register = context
                    .register_map
                    .get(rhs)
                    .ok_or_else(|| format!("Register {} not found", rhs))?
                    .clone();

                  context.text_section.statements.push(match operator {
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
                  let lhs_register = context
                    .register_map
                    .get(lhs)
                    .ok_or_else(|| format!("Register {} not found", lhs))?
                    .clone();
                  let rhs_value = rhs.as_immediate()?;

                  context.text_section.statements.push(match operator {
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
                  let lhs_register = new_register(&mut context.register_map);
                  load_immediate(&mut context.text_section, lhs_register.clone(), lhs_value);

                  context.text_section.statements.push(match operator {
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
              } => {
                if operation_type != VarType::Bool {
                  return Err("Conditional operations must be of type bool".to_string());
                }

                if is_register(&lhs) && is_register(&rhs) {
                  let lhs = lhs.as_identifier()?;
                  let rhs = rhs.as_identifier()?;

                  let lhs_register = context
                    .register_map
                    .get(lhs)
                    .ok_or_else(|| format!("Register {} not found", lhs))?
                    .clone();
                  let rhs_register = context
                    .register_map
                    .get(rhs)
                    .ok_or_else(|| format!("Register {} not found", rhs))?
                    .clone();

                  context.text_section.statements.push(match condition {
                    Condition::LessThan => {
                      create_instruction!(
                        Instruction::Slt,
                        register,
                        lhs_register,
                        InstructionArgument::Register(Register { name: rhs_register })
                      )
                    }
                    Condition::GreaterThan => todo!(),
                    Condition::LessThanOrEqual => todo!(),
                    Condition::GreaterThanOrEqual => todo!(),
                    Condition::Equal => todo!(),
                    Condition::NotEqual => todo!(),
                    Condition::And => todo!(),
                    Condition::Or => todo!(),
                  });
                } else if is_register(&lhs) && is_immediate(&rhs) {
                  let lhs = lhs.as_identifier()?;
                  let lhs_register = context
                    .register_map
                    .get(lhs)
                    .ok_or_else(|| format!("Register {} not found", lhs))?
                    .clone();
                  let rhs_value = rhs.as_immediate()?;

                  context.text_section.statements.push(match condition {
                    Condition::LessThan => {
                      create_instruction!(
                        Instruction::Slt,
                        register,
                        lhs_register,
                        InstructionArgument::Immediate(rhs_value)
                      )
                    }
                    Condition::GreaterThan => todo!(),
                    Condition::LessThanOrEqual => todo!(),
                    Condition::GreaterThanOrEqual => todo!(),
                    Condition::Equal => todo!(),
                    Condition::NotEqual => todo!(),
                    Condition::And => todo!(),
                    Condition::Or => todo!(),
                  });
                }
              }
            },
            crate::ast::Expr::FunctionCall(_) => todo!(),
          }
        }
        CompassStatement::ConditionalJump {
          condition,
          label,
          location,
        } => match condition {
          Expr::BinaryOperation(op) => match op {
            BinaryOperation::Conditional {
              ref lhs,
              ref condition,
              ref rhs,
              operation_type,
            } => {
              if operation_type != VarType::Bool {
                return Err("Conditional operations must be of type bool".to_string());
              }

              if is_register(&lhs) && is_register(&rhs) {
                let lhs = lhs.as_identifier()?;
                let rhs = rhs.as_identifier()?;

                let lhs_register = context
                  .register_map
                  .get(lhs)
                  .ok_or_else(|| format!("Register {} not found", lhs))?
                  .clone();
                let rhs_register = context
                  .register_map
                  .get(rhs)
                  .ok_or_else(|| format!("Register {} not found", rhs))?
                  .clone();

                let condition_instruction = match condition {
                  Condition::LessThan => Instruction::Blt,
                  Condition::GreaterThan => Instruction::Bgt,
                  Condition::LessThanOrEqual => Instruction::Ble,
                  Condition::GreaterThanOrEqual => Instruction::Bge,
                  Condition::Equal => Instruction::Beq,
                  Condition::NotEqual => Instruction::Bne,
                  _ => Err(format!(
                    "Invalid binary operation for conditional jump {op:?}",
                  ))?,
                };

                context.text_section.statements.push(create_instruction!(
                  condition_instruction,
                  lhs_register,
                  rhs_register,
                  InstructionArgument::Label(label)
                ));
              } else if is_register(&lhs) && is_immediate(&rhs) {
                let lhs = lhs.as_identifier()?;
                let lhs_register = context
                  .register_map
                  .get(lhs)
                  .ok_or_else(|| format!("Register {} not found", lhs))?
                  .clone();
                let rhs_value = rhs.as_immediate()?;

                let condition_instruction = match condition {
                  Condition::LessThan => Instruction::Blt,
                  Condition::GreaterThan => Instruction::Bgt,
                  Condition::LessThanOrEqual => Instruction::Ble,
                  Condition::GreaterThanOrEqual => Instruction::Bge,
                  Condition::Equal => Instruction::Beq,
                  Condition::NotEqual => Instruction::Bne,
                  _ => Err(format!(
                    "Invalid binary operation for conditional jump {op:?}",
                  ))?,
                };

                context.text_section.statements.push(Statement::Instruction(
                  condition_instruction(
                    [
                      InstructionArgument::Register(Register { name: lhs_register }),
                      InstructionArgument::Immediate(rhs_value),
                      InstructionArgument::Label(label),
                    ]
                    .into(),
                  ),
                ));
              }
            }
            _ => Err(format!(
              "Invalid binary operation for conditional jump {op:?}",
            ))?,
          },
          Expr::Operand(op) => match op {
            Operand::Identifier(ident) => {
              let register = context
                .register_map
                .get(&ident)
                .ok_or_else(|| format!("Register {} not found", ident))?
                .clone();

              context
                .text_section
                .statements
                .push(Statement::Instruction(Instruction::Beqz(
                  [
                    InstructionArgument::Register(Register {
                      name: register.clone(),
                    }),
                    InstructionArgument::Label(label),
                  ]
                  .into(),
                )));
            }
            Operand::LiteralBool(value) => match value {
              true => context
                .text_section
                .statements
                .push(Statement::Instruction(Instruction::J(
                  [InstructionArgument::Label(label)].into(),
                ))),
              false => (),
            },
            _ => Err(format!("Invalid operand for conditional jump {}", op))?,
          },
          Expr::FunctionCall(_) => unreachable!(),
        },
        CompassStatement::UnconditionalJump { label, location } => {
          context
            .text_section
            .statements
            .push(Statement::Instruction(Instruction::J(
              [InstructionArgument::Label(label)].into(),
            )));
        }
        CompassStatement::Label { name, location } => {
          if name == "main" {
            return Err("Cannot use 'main' as a label name".to_string());
          }

          context.text_section.statements.push(Statement::Label(name))
        }
        CompassStatement::FunctionDefinition(function) => {
          let name = format!("__{name}", name = function.name.clone());

          context.function_map.insert(name.clone(), function.clone());

          // Function declarations should be added before the `main: flow`
          let statements = vec![Statement::Label(name)];

          // TODO: Insert variables into the scope

          let mut save_statements = context.text_section.statements.clone();
          context.text_section.statements = statements;
          context.scope_level += 1;
          self.generate(function.body, context)?;
          context.scope_level -= 1;

          context
            .text_section
            .statements
            .push(Statement::Instruction(Instruction::Jr(
              [InstructionArgument::Register(Register {
                name: "$ra".to_string(),
              })]
              .into(),
            )));

          context.text_section.statements.append(&mut save_statements);
        }
        CompassStatement::Store { at, from, location } => match (&at, &from) {
          (Operand::Dereference(at), Operand::Identifier(from)) => {
            let at_register = context
              .register_map
              .get(at)
              .ok_or_else(|| format!("Register {} not found", at))?
              .clone();
            let from_register = context
              .register_map
              .get(from)
              .ok_or_else(|| format!("Register {} not found", from))?
              .clone();

            context
              .text_section
              .statements
              .push(Statement::Instruction(Instruction::Sw(
                [
                  InstructionArgument::Register(Register {
                    name: from_register,
                  }),
                  InstructionArgument::Register(Register { name: at_register }),
                ]
                .into(),
              )));
          }
          _ => {
            Err(format!(
              "Invalid operands for store operation {} and {}",
              at, from
            ))?;
          }
        },
        CompassStatement::NoOperation => {}
        CompassStatement::Call {
          name,
          params,
          return_type,
          location,
        } => {
          let function = context
            .get_function(&name)
            .ok_or_else(|| format!("Function {} not found", name))?;

          if function.is_builtin {
            match function.name.as_str() {
              "write_string" => {
                // Perform the write_string syscall (v0 = 4, a0 = string address)
                context
                  .text_section
                  .statements
                  .push(Statement::Instruction(Instruction::Li(
                    [
                      InstructionArgument::Register(Register {
                        name: "$v0".to_string(),
                      }),
                      InstructionArgument::Immediate(4),
                    ]
                    .into(),
                  )));

                let string_register = match &params[0] {
                  Operand::Identifier(ident) => context
                    .register_map
                    .get(ident)
                    .ok_or_else(|| format!("Register {} not found", ident))?
                    .clone(),
                  Operand::LiteralStr(str) => {
                    let register = new_register(&mut context.register_map);
                    load_string(
                      &mut context.text_section,
                      &mut context.data_section,
                      register.clone(),
                      str.clone(),
                    );

                    register
                  }
                  _ => todo!(),
                };

                context
                  .text_section
                  .statements
                  .push(Statement::Instruction(Instruction::Move(
                    [
                      InstructionArgument::Register(Register {
                        name: "$a0".to_string(),
                      }),
                      InstructionArgument::Register(Register {
                        name: string_register,
                      }),
                    ]
                    .into(),
                  )));

                context
                  .text_section
                  .statements
                  .push(Statement::Instruction(Instruction::Syscall));
              }
              _ => todo!(),
            }
          } else {
            for (i, param) in params.iter().enumerate() {
              let register = match param {
                Operand::Identifier(ident) => {
                  find_or_create_reg(&mut context.register_map, ident.clone())
                }
                Operand::LiteralStr(str) => {
                  let register = new_register(&mut context.register_map);
                  load_string(
                    &mut context.text_section,
                    &mut context.data_section,
                    register.clone(),
                    str.clone(),
                  );

                  register
                }
                Operand::LiteralBool(value) => {
                  load_immediate_to_new_register(context, *value as u32)
                }
                Operand::LiteralI8(value) => load_immediate_to_new_register(context, *value as u32),
                Operand::LiteralI16(value) => {
                  load_immediate_to_new_register(context, *value as u32)
                }
                Operand::LiteralI32(value) => {
                  load_immediate_to_new_register(context, *value as u32)
                }
                Operand::LiteralI64(value) => {
                  load_immediate_to_new_register(context, *value as u32)
                }
                Operand::LiteralU8(value) => load_immediate_to_new_register(context, *value as u32),
                Operand::LiteralU16(value) => {
                  load_immediate_to_new_register(context, *value as u32)
                }
                Operand::LiteralU32(value) => load_immediate_to_new_register(context, *value),
                Operand::LiteralU64(value) => {
                  load_immediate_to_new_register(context, *value as u32)
                }
                Operand::LiteralF32(_) => todo!(),
                Operand::LiteralF64(_) => todo!(),
                Operand::Dereference(_) => todo!(),
              };

              context
                .text_section
                .statements
                .push(Statement::Instruction(Instruction::Move(
                  [
                    InstructionArgument::Register(Register {
                      name: format!("a{}", i),
                    }),
                    InstructionArgument::Register(Register { name: register }),
                  ]
                  .into(),
                )));
            }

            context
              .text_section
              .statements
              .push(Statement::Instruction(Instruction::Jal(
                [InstructionArgument::Label(name)].into(),
              )));
          }
        }
      }
    }

    let program = Program {
      data_section: context.data_section.clone(),
      text_section: context.text_section.clone(),
    };

    Ok(program.to_string())
  }
}

fn load_immediate_to_new_register(context: &mut Context, value: u32) -> String {
  let register = new_register(&mut context.register_map);
  load_immediate(&mut context.text_section, register.clone(), value);
  register
}

fn find_or_create_reg(register_map: &mut HashMap<String, String>, name: String) -> String {
  if let Some(register) = register_map.get(&name) {
    register.clone()
  } else {
    let register = format!("$t{}", register_map.len());
    register_map.insert(name.clone(), register.clone());
    register
  }
}

fn new_register(register_map: &mut HashMap<String, String>) -> String {
  let register = format!("$t{}", register_map.len());
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
