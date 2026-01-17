use crate::{
    bytecode::bytecode::OpCode,
    errors::HydorError,
    hydor_vm::vm::{BOOLEAN_FALSE, BOOLEAN_TRUE, HydorVM},
    runtime_value::RuntimeValue,
    utils::Span,
};

impl HydorVM {
    pub(crate) fn unary_operation(&mut self, opcode: OpCode, span: Span) -> Result<(), HydorError> {
        match opcode {
            OpCode::UnaryNegate => self.unary_negation_operation(span),
            OpCode::UnaryNot => self.unary_not_operation(),

            _ => unreachable!(),
        }
    }

    pub(crate) fn unary_negation_operation(&mut self, span: Span) -> Result<(), HydorError> {
        // This does a direct stack modification
        // which is faster than popping and pushing
        // a value into the stack
        let target = self.peek_offset(0)?;
        let target_span = self.peek_span(0)?;

        if !target.is_number() {
            // Merge the operator span with the operand span
            let full_span = Span {
                line: span.line,
                start_column: span.start_column,
                end_column: target_span.end_column,
            };

            return Err(HydorError::UnaryOperationError {
                operation: "negation".to_string(),
                operand_type: target.get_type(),
                span: full_span,
            });
        }

        if !target.is_number() {
            return Err(HydorError::UnaryOperationError {
                operation: "negation".to_string(),
                operand_type: target.get_type(),
                span: span,
            });
        }

        if target.is_float() {
            let lit = target.as_float().unwrap();
            self.set_offset_value(0, RuntimeValue::FloatLiteral(-lit))?; // Negate it!
        } else {
            let lit = target.as_int().unwrap();
            self.set_offset_value(0, RuntimeValue::IntegerLiteral(-lit))?; // Negate it!
        }

        Ok(())
    }

    pub(crate) fn unary_not_operation(&mut self) -> Result<(), HydorError> {
        let target = self.peek_offset(0)?;

        // Just flip return type
        if self.is_truthy(target) {
            self.set_offset_value(0, BOOLEAN_FALSE)?;
        } else {
            self.set_offset_value(0, BOOLEAN_TRUE)?;
        }

        Ok(())
    }
}
