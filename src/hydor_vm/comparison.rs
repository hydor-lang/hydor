use crate::{
    bytecode::bytecode::OpCode,
    errors::HydorError,
    hydor_vm::{
        helpers::opcode_to_operator,
        vm::{BOOLEAN_FALSE, BOOLEAN_TRUE, HydorVM},
    },
    runtime_value::RuntimeValue,
    utils::Span,
};

impl HydorVM {
    pub(crate) fn compare_operation(
        &mut self,
        opcode: OpCode,
        span: Span,
    ) -> Result<(), HydorError> {
        let (right_val, right_span) = self.pop_with_span()?;
        let (left_val, left_span) = self.pop_with_span()?;

        // Number comparison
        if left_val.is_number() && right_val.is_number() {
            return self.compare_numbers(opcode, left_val, right_val, span);
        }

        // Only == and != are allowed for non-numeric types
        match opcode {
            OpCode::CompareEqual => {
                let result = self.values_equal(left_val, right_val);
                self.push(if result { BOOLEAN_TRUE } else { BOOLEAN_FALSE }, span)
            }
            OpCode::CompareNotEqual => {
                let result = self.values_equal(left_val, right_val);
                self.push(if result { BOOLEAN_FALSE } else { BOOLEAN_TRUE }, span)
            }
            _ => {
                // <, <=, >, >= require numbers
                let blame_type = if !left_val.is_number() {
                    left_val.get_type()
                } else {
                    right_val.get_type()
                };

                Err(HydorError::ComparisonOperationError {
                    operation: opcode_to_operator(opcode),
                    blame_type,
                    span,
                })
            }
        }
    }

    pub(crate) fn compare_numbers(
        &mut self,
        opcode: OpCode,
        left: RuntimeValue,
        right: RuntimeValue,
        span: Span,
    ) -> Result<(), HydorError> {
        let left_num = left.as_number().unwrap();
        let right_num = right.as_number().unwrap();

        let result = match opcode {
            OpCode::CompareLess => left_num < right_num,
            OpCode::CompareLessEqual => left_num <= right_num,
            OpCode::CompareGreater => left_num > right_num,
            OpCode::CompareGreaterEqual => left_num >= right_num,
            OpCode::CompareEqual => left_num == right_num,
            OpCode::CompareNotEqual => left_num != right_num,
            _ => unreachable!(),
        };

        self.push(if result { BOOLEAN_TRUE } else { BOOLEAN_FALSE }, span)
    }

    pub(crate) fn values_equal(&self, left: RuntimeValue, right: RuntimeValue) -> bool {
        match (left, right) {
            (RuntimeValue::IntegerLiteral(a), RuntimeValue::IntegerLiteral(b)) => a == b,
            (RuntimeValue::FloatLiteral(a), RuntimeValue::FloatLiteral(b)) => a == b,
            (RuntimeValue::BooleanLiteral(a), RuntimeValue::BooleanLiteral(b)) => a == b,
            (RuntimeValue::StringLiteral(a), RuntimeValue::StringLiteral(b)) => a == b,
            (RuntimeValue::NilLiteral, RuntimeValue::NilLiteral) => true,

            // Allow int/float comparison
            (RuntimeValue::IntegerLiteral(a), RuntimeValue::FloatLiteral(b)) => (a as f64) == b,
            (RuntimeValue::FloatLiteral(a), RuntimeValue::IntegerLiteral(b)) => a == (b as f64),

            _ => false,
        }
    }
}
