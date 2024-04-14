use celestial_hub_astrolabe::{
  ast::{
    DataSection, Instruction, InstructionArgument, Program, Register, Statement, TextSection,
    Variable,
  },
  lexer::{
    tokens::{Type, Value},
    traits::iterator,
  },
};

use crate::ast::{
  context, BinaryOperation, Condition, Expr, Function, FunctionCall, Operand, Operator,
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
              Operand::LiteralBool(val) => {
                load_immediate(&mut context.text_section, register, val as u32)
              }
              Operand::Identifier(var) => {
                let var_register = context
                  .register_map
                  .get(&var)
                  .ok_or_else(|| format!("Register {} not found", var))?
                  .clone();

                context
                  .text_section
                  .statements
                  .push(Statement::Instruction(Instruction::Move(
                    [
                      InstructionArgument::Register(Register {
                        name: register.clone(),
                      }),
                      InstructionArgument::Register(Register { name: var_register }),
                    ]
                    .into(),
                  )));
              }
              // TODO: Handle float literals (should be in a different register)
              Operand::LiteralF32(val) => todo!(),
              Operand::LiteralF64(val) => todo!(),
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
                    Operator::Mul => create_instruction!(
                      Instruction::Mul,
                      register,
                      lhs_register,
                      InstructionArgument::Register(Register { name: rhs_register })
                    ),
                    Operator::Div => create_instruction!(
                      Instruction::Div,
                      register,
                      lhs_register,
                      InstructionArgument::Register(Register { name: rhs_register })
                    ),
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
                      Instruction::Add,
                      register,
                      lhs_register,
                      InstructionArgument::Immediate(rhs_value)
                    ),
                    Operator::Sub => create_instruction!(
                      Instruction::Sub,
                      register,
                      lhs_register,
                      InstructionArgument::Immediate(rhs_value)
                    ),
                    Operator::Mul => create_instruction!(
                      Instruction::Mul,
                      register,
                      lhs_register,
                      InstructionArgument::Immediate(rhs_value)
                    ),
                    Operator::Div => create_instruction!(
                      Instruction::Div,
                      register,
                      lhs_register,
                      InstructionArgument::Immediate(rhs_value)
                    ),
                  });
                } else if is_immediate(&lhs) && is_immediate(&rhs) {
                  let rhs = rhs.as_immediate()?;
                  let lhs_value = lhs.as_immediate()?;

                  // There is no instruction that can add two immediates, so we need to load one of them into a register first
                  let lhs_register = new_register(&mut context.register_map);
                  load_immediate(&mut context.text_section, lhs_register.clone(), lhs_value);

                  context.text_section.statements.push(match operator {
                    Operator::Add => create_instruction!(
                      Instruction::Add,
                      register,
                      lhs_register,
                      InstructionArgument::Immediate(rhs)
                    ),
                    Operator::Sub => create_instruction!(
                      Instruction::Sub,
                      register,
                      lhs_register,
                      InstructionArgument::Immediate(rhs)
                    ),
                    Operator::Mul => create_instruction!(
                      Instruction::Mul,
                      register,
                      lhs_register,
                      InstructionArgument::Immediate(rhs)
                    ),
                    Operator::Div => create_instruction!(
                      Instruction::Div,
                      register,
                      lhs_register,
                      InstructionArgument::Immediate(rhs)
                    ),
                    _ => todo!(),
                  });
                } else {
                  Err(format!(
                    "Invalid operands for arithmetic operation {} and {}",
                    lhs, rhs
                  ))?;
                }
              }
              BinaryOperation::Conditional {
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
                  let instruction = match condition {
                    Condition::LessThan => Instruction::Slt,
                    Condition::GreaterThan => Instruction::Sgt,
                    Condition::LessThanOrEqual => Instruction::Sle,
                    Condition::GreaterThanOrEqual => Instruction::Sge,
                    Condition::Equal => Instruction::Seq,
                    Condition::NotEqual => Instruction::Sne,
                    Condition::And | Condition::Or => {
                      return Err(
                        "Cannot perform logical operations on immediate values".to_string(),
                      );
                    }
                  };
                  context.text_section.statements.push(create_instruction!(
                    instruction,
                    register,
                    lhs_register,
                    InstructionArgument::Register(Register { name: rhs_register })
                  ));
                } else if is_register(&lhs) && is_immediate(&rhs) {
                  let lhs = lhs.as_identifier()?;
                  let lhs_register = context
                    .register_map
                    .get(lhs)
                    .ok_or_else(|| format!("Register {} not found", lhs))?
                    .clone();
                  let rhs_value = rhs.as_immediate()?;

                  let instruction = match condition {
                    Condition::LessThan => Instruction::Slt,
                    Condition::GreaterThan => Instruction::Sgt,
                    Condition::LessThanOrEqual => Instruction::Sle,
                    Condition::GreaterThanOrEqual => Instruction::Sge,
                    Condition::Equal => Instruction::Seq,
                    Condition::NotEqual => Instruction::Sne,
                    Condition::And | Condition::Or => {
                      return Err(
                        "Cannot perform logical operations on immediate values".to_string(),
                      );
                    }
                  };

                  context.text_section.statements.push(create_instruction!(
                    instruction,
                    register,
                    lhs_register,
                    InstructionArgument::Immediate(rhs_value)
                  ));
                }
              }
            },
            Expr::FunctionCall(function_call) => {
              let function = context
                .get_function(&function_call.name)
                .ok_or_else(|| format!("Function {} not found", function_call.name))?;

              if function.is_builtin {
                match function.name.as_str() {
                  "read_int" => {
                    load_immediate(&mut context.text_section, "$v0".to_string(), 5);
                  }
                  "read_string" => {
                    // $a0 = address of the buffer
                    // $a1 = length of the buffer

                    load_immediate(&mut context.text_section, "$v0".to_string(), 8);

                    let size = if let Operand::LiteralU32(size) = &function_call.params[0] {
                      *size
                    } else {
                      return Err("Invalid argument for read_string".to_string());
                    };

                    context.buffer_counter += 1;
                    let label = format!("__buffer_{label}", label = context.buffer_counter);
                    context.data_section.variables.push(Variable {
                      name: label.clone(),
                      type_: Type::Space,
                      value: Value::Bytes(size),
                    });

                    context.text_section.statements.append(
                      &mut [
                        Statement::Instruction(Instruction::La(
                          [
                            InstructionArgument::Register(Register {
                              name: "$a0".to_string(),
                            }),
                            InstructionArgument::Label(label),
                          ]
                          .into(),
                        )),
                        Statement::Instruction(Instruction::Li(
                          [
                            InstructionArgument::Register(Register {
                              name: "$a1".to_string(),
                            }),
                            InstructionArgument::Immediate(size),
                          ]
                          .into(),
                        )),
                      ]
                      .into(),
                    );
                  }
                  _ => Err(format!("Function {} not found", function.name))?,
                };

                context.text_section.statements.append(
                  &mut [
                    // Perform the syscall
                    Statement::Instruction(Instruction::Syscall),
                    // Move the result of the syscall to the register
                    Statement::Instruction(Instruction::Move(
                      [
                        InstructionArgument::Register(Register {
                          name: register.clone(),
                        }),
                        InstructionArgument::Register(Register {
                          name: "$v0".to_string(),
                        }),
                      ]
                      .into(),
                    )),
                  ]
                  .into(),
                );
              } else {
                function_call
                  .params
                  .iter()
                  .enumerate()
                  .for_each(|(i, param)| {
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
                      Operand::LiteralI8(value) => {
                        load_immediate_to_new_register(context, *value as u32)
                      }
                      Operand::LiteralI16(value) => {
                        load_immediate_to_new_register(context, *value as u32)
                      }
                      Operand::LiteralI32(value) => {
                        load_immediate_to_new_register(context, *value as u32)
                      }
                      Operand::LiteralI64(value) => {
                        load_immediate_to_new_register(context, *value as u32)
                      }
                      Operand::LiteralU8(value) => {
                        load_immediate_to_new_register(context, *value as u32)
                      }
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

                    context.text_section.statements.push(Statement::Instruction(
                      Instruction::Move(
                        [
                          InstructionArgument::Register(Register {
                            name: format!("$a{}", i),
                          }),
                          InstructionArgument::Register(Register { name: register }),
                        ]
                        .into(),
                      ),
                    ));
                  });
              }
            }
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

                match condition {
                  Condition::And => {
                    context.conditional_counter += 1;
                    let new_label = format!("__and_{label}", label = context.conditional_counter);

                    context.text_section.statements.append(
                      &mut [
                        Statement::Instruction(Instruction::Beqz(
                          [
                            InstructionArgument::Register(Register {
                              name: lhs_register.clone(),
                            }),
                            InstructionArgument::Label(new_label.clone()),
                          ]
                          .into(),
                        )),
                        Statement::Instruction(Instruction::Beqz(
                          [
                            InstructionArgument::Register(Register {
                              name: rhs_register.clone(),
                            }),
                            InstructionArgument::Label(new_label.clone()),
                          ]
                          .into(),
                        )),
                        Statement::Instruction(Instruction::J(
                          [InstructionArgument::Label(label.clone())].into(),
                        )),
                        Statement::Label(new_label),
                      ]
                      .into(),
                    );
                  }
                  Condition::Or => {
                    context.text_section.statements.append(
                      &mut [
                        Statement::Instruction(Instruction::Bnez(
                          [
                            InstructionArgument::Register(Register {
                              name: lhs_register.clone(),
                            }),
                            InstructionArgument::Label(label.clone()),
                          ]
                          .into(),
                        )),
                        Statement::Instruction(Instruction::Bnez(
                          [
                            InstructionArgument::Register(Register {
                              name: rhs_register.clone(),
                            }),
                            InstructionArgument::Label(label.clone()),
                          ]
                          .into(),
                        )),
                      ]
                      .into(),
                    );
                  }
                  _ => {
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
                  }
                };
              } else if is_register(&lhs) && is_immediate(&rhs) {
                let lhs = lhs.as_identifier()?;
                let lhs_register = context
                  .register_map
                  .get(lhs)
                  .ok_or_else(|| format!("Register {} not found", lhs))?
                  .clone();
                let rhs_value = rhs.as_immediate()?;

                match condition {
                  Condition::And => {
                    context.conditional_counter += 1;
                    let new_label = format!("__and_{label}", label = context.conditional_counter);

                    context.text_section.statements.append(
                      &mut [
                        Statement::Instruction(Instruction::Beqz(
                          [
                            InstructionArgument::Register(Register {
                              name: lhs_register.clone(),
                            }),
                            InstructionArgument::Label(new_label.clone()),
                          ]
                          .into(),
                        )),
                        Statement::Instruction(Instruction::Beqz(
                          [
                            InstructionArgument::Immediate(rhs_value),
                            InstructionArgument::Label(new_label.clone()),
                          ]
                          .into(),
                        )),
                        Statement::Instruction(Instruction::J(
                          [InstructionArgument::Label(label.clone())].into(),
                        )),
                        Statement::Label(new_label),
                      ]
                      .into(),
                    );
                  }
                  Condition::Or => {
                    context.text_section.statements.append(
                      &mut [
                        Statement::Instruction(Instruction::Bnez(
                          [
                            InstructionArgument::Register(Register {
                              name: lhs_register.clone(),
                            }),
                            InstructionArgument::Label(label.clone()),
                          ]
                          .into(),
                        )),
                        Statement::Instruction(Instruction::Bnez(
                          [
                            InstructionArgument::Immediate(rhs_value),
                            InstructionArgument::Label(label.clone()),
                          ]
                          .into(),
                        )),
                      ]
                      .into(),
                    );
                  }
                  _ => {
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
                };
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
        CompassStatement::Call(FunctionCall {
          name,
          params,
          return_type,
          location,
        }) => {
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
              "write_int" => {
                // Perform the write_int syscall (v0 = 1, a0 = integer value)
                context
                  .text_section
                  .statements
                  .push(Statement::Instruction(Instruction::Li(
                    [
                      InstructionArgument::Register(Register {
                        name: "$v0".to_string(),
                      }),
                      InstructionArgument::Immediate(1),
                    ]
                    .into(),
                  )));

                let int_register = match &params[0] {
                  Operand::Identifier(ident) => context
                    .register_map
                    .get(ident)
                    .ok_or_else(|| format!("Register {} not found", ident))?
                    .clone(),
                  Operand::LiteralI8(val) => load_immediate_to_new_register(context, *val as u32),
                  Operand::LiteralI16(val) => load_immediate_to_new_register(context, *val as u32),
                  Operand::LiteralI32(val) => load_immediate_to_new_register(context, *val as u32),
                  Operand::LiteralI64(val) => load_immediate_to_new_register(context, *val as u32),
                  Operand::LiteralU8(val) => load_immediate_to_new_register(context, *val as u32),
                  Operand::LiteralU16(val) => load_immediate_to_new_register(context, *val as u32),
                  Operand::LiteralU32(val) => load_immediate_to_new_register(context, *val),
                  Operand::LiteralU64(val) => load_immediate_to_new_register(context, *val as u32),
                  _ => todo!(),
                };

                context.text_section.statements.append(
                  &mut [
                    Statement::Instruction(Instruction::Move(
                      [
                        InstructionArgument::Register(Register {
                          name: "$a0".to_string(),
                        }),
                        InstructionArgument::Register(Register { name: int_register }),
                      ]
                      .into(),
                    )),
                    Statement::Instruction(Instruction::Syscall),
                  ]
                  .into(),
                );
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
                      name: format!("$a{}", i),
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
        CompassStatement::NoOperation => {}
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
    Operand::Dereference(_) => true,
    _ => false,
  }
}

fn is_immediate(value: &crate::ast::Operand) -> bool {
  match value {
    Operand::Identifier(_) => false,
    Operand::Dereference(_) => false,
    _ => true,
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
