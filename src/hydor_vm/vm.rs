use crate::{
    bytecode::bytecode::{Instructions, OpCode, ToOpcode, read_uint16},
    compiler::compiler::{Bytecode, DebugInfo},
    errors::HydorError,
    runtime_value::RuntimeValue,
    utils::Span,
};

const MAX_STACK: usize = 10_000;

pub struct HydorVM {
    stack: Vec<StackValue>,
    last_pop: Option<RuntimeValue>,

    pub instructions: Instructions,
    pub ip: usize,

    pub string_table: Vec<String>,
    pub constants: Vec<RuntimeValue>,

    debug_info: DebugInfo,
}

#[derive(Debug, Clone, Copy)]
struct StackValue {
    value: RuntimeValue,
    span: Span,
}

// SINGLETON ---
pub const BOOLEAN_TRUE: RuntimeValue = RuntimeValue::BooleanLiteral(true);
pub const BOOLEAN_FALSE: RuntimeValue = RuntimeValue::BooleanLiteral(false);
pub const NIL_LITERAL: RuntimeValue = RuntimeValue::NilLiteral;

impl HydorVM {
    pub fn new(bytecode: Bytecode) -> Self {
        Self {
            stack: Vec::with_capacity(MAX_STACK),
            last_pop: None,

            string_table: bytecode.string_table,
            instructions: bytecode.instructions,
            ip: 0,

            constants: bytecode.constants,
            debug_info: bytecode.debug_info,
        }
    }

    /// Main entry point
    pub fn execute_bytecode(&mut self) -> Result<(), HydorError> {
        while self.ip < self.instructions.len() {
            let opcode = self.instructions[self.ip].to_opcode();
            let span = self.debug_info.get_span(self.ip);

            match opcode {
                OpCode::LoadConstant => {
                    self.load_constant(span)?;
                }

                OpCode::LoadString => {
                    self.load_string(span)?;
                }

                OpCode::LoadNil => {
                    self.push(NIL_LITERAL, span)?;
                }
                OpCode::LoadBoolTrue => {
                    self.push(BOOLEAN_TRUE, span)?;
                }
                OpCode::LoadBoolFalse => {
                    self.push(BOOLEAN_FALSE, span)?;
                }

                OpCode::Add => {
                    self.binary_op_add()?;
                }
                OpCode::Subtract => {
                    self.binary_op_numeric("subtraction", |a, b| a - b)?;
                }
                OpCode::Multiply => {
                    self.binary_op_numeric("multiplication", |a, b| a * b)?;
                }
                OpCode::Divide => {
                    self.binary_op_numeric("division", |a, b| a / b)?;
                }
                OpCode::Exponent => {
                    self.binary_op_numeric("exponentiation", |a, b| a.powf(b))?;
                }

                OpCode::UnaryNegate => {
                    self.unary_operation(opcode, span)?;
                }

                OpCode::UnaryNot => {
                    self.unary_operation(opcode, span)?;
                }

                OpCode::CompareLess => {
                    self.compare_operation(opcode, span)?;
                }
                OpCode::CompareLessEqual => {
                    self.compare_operation(opcode, span)?;
                }
                OpCode::CompareGreater => {
                    self.compare_operation(opcode, span)?;
                }
                OpCode::CompareGreaterEqual => {
                    self.compare_operation(opcode, span)?;
                }
                OpCode::CompareEqual => {
                    self.compare_operation(opcode, span)?;
                }
                OpCode::CompareNotEqual => {
                    self.compare_operation(opcode, span)?;
                }

                OpCode::Pop => {
                    self.last_pop = Some(self.pop_value()?);
                }
                OpCode::Halt => {
                    return Ok(());
                }
            }

            self.ip += 1; // Advance opcode
        }

        unreachable!()
    }

    pub(crate) fn push(&mut self, value: RuntimeValue, span: Span) -> Result<(), HydorError> {
        if self.stack.len() >= MAX_STACK {
            return Err(HydorError::StackOverflow {
                stack_length: self.stack.len(),
                span,
            });
        }

        self.stack.push(StackValue { value, span });
        Ok(())
    }

    pub(crate) fn peek_offset(&self, n: usize) -> Result<RuntimeValue, HydorError> {
        let size = self.stack.len();
        if n >= size {
            return Err(HydorError::StackUnderflow {
                stack_length: size,
                span: Span::default(),
            });
        }

        Ok(self.stack[size - 1 - n].value)
    }

    pub(crate) fn peek_span(&self, n: usize) -> Result<Span, HydorError> {
        let size = self.stack.len();
        if n >= size {
            return Err(HydorError::StackUnderflow {
                stack_length: size,
                span: Span::default(),
            });
        }

        Ok(self.stack[size - 1 - n].span)
    }

    pub(crate) fn set_offset_value(
        &mut self,
        n: usize,
        new_value: RuntimeValue,
    ) -> Result<(), HydorError> {
        let size = self.stack.len();
        if n >= size {
            return Err(HydorError::StackUnderflow {
                stack_length: size,
                span: Span::default(),
            });
        }

        self.stack[size - 1 - n].value = new_value;
        Ok(())
    }

    pub(crate) fn pop_value(&mut self) -> Result<RuntimeValue, HydorError> {
        self.stack
            .pop()
            .map(|sv| sv.value)
            .ok_or(HydorError::StackUnderflow {
                stack_length: 0,
                span: Span::default(),
            })
    }

    pub(crate) fn pop_with_span(&mut self) -> Result<(RuntimeValue, Span), HydorError> {
        self.stack
            .pop()
            .map(|sv| (sv.value, sv.span))
            .ok_or(HydorError::StackUnderflow {
                stack_length: 0,
                span: Span::default(),
            })
    }

    // For reading only
    pub fn resolve_string(&self, index: usize) -> &str {
        &self.string_table[index]
    }

    pub fn last_popped(&self) -> Option<RuntimeValue> {
        self.last_pop
    }
}
