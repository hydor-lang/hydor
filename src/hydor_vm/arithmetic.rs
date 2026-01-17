use crate::{errors::HydorError, hydor_vm::vm::HydorVM, runtime_value::RuntimeValue, utils::Span};

impl HydorVM {
    pub(crate) fn binary_op_add(&mut self) -> Result<(), HydorError> {
        let (right, right_span) = self.pop_with_span()?;
        let (left, left_span) = self.pop_with_span()?;

        // String concatenation
        if matches!(left, RuntimeValue::StringLiteral(_))
            && matches!(right, RuntimeValue::StringLiteral(_))
        {
            return self.string_concat(left, left_span, right, right_span);
        }

        // Numeric addition
        if !left.is_number() {
            return Err(HydorError::ArithmeticError {
                operation: "addition".to_string(),
                left_type: left.get_type(),
                right_type: right.get_type(),
                span: left_span,
            });
        }

        if !right.is_number() {
            return Err(HydorError::ArithmeticError {
                operation: "addition".to_string(),
                left_type: left.get_type(),
                right_type: right.get_type(),
                span: right_span,
            });
        }

        let result = self.compute_numeric(left, right, |a, b| a + b);
        let result_span = Span {
            line: left_span.line,
            start_column: left_span.start_column,
            end_column: right_span.end_column,
        };

        self.push(result, result_span)?;
        Ok(())
    }

    /// Generic numeric binary operation
    pub(crate) fn binary_op_numeric<F>(&mut self, op_name: &str, f: F) -> Result<(), HydorError>
    where
        F: Fn(f64, f64) -> f64,
    {
        let (right, right_span) = self.pop_with_span()?;
        let (left, left_span) = self.pop_with_span()?;

        if !left.is_number() {
            return Err(HydorError::ArithmeticError {
                operation: op_name.to_string(),
                left_type: left.get_type(),
                right_type: right.get_type(),
                span: left_span,
            });
        }

        if !right.is_number() {
            return Err(HydorError::ArithmeticError {
                operation: op_name.to_string(),
                left_type: left.get_type(),
                right_type: right.get_type(),
                span: right_span,
            });
        }

        let result = self.compute_numeric(left, right, f);
        let result_span = Span {
            line: left_span.line,
            start_column: left_span.start_column,
            end_column: right_span.end_column,
        };

        self.push(result, result_span)?;
        Ok(())
    }

    /// Compute numeric operation and preserve int/float types when possible
    pub(crate) fn compute_numeric<F>(&self, left: RuntimeValue, right: RuntimeValue, f: F) -> RuntimeValue
    where
        F: Fn(f64, f64) -> f64,
    {
        let a = match left {
            RuntimeValue::IntegerLiteral(n) => n as f64,
            RuntimeValue::FloatLiteral(n) => n,
            _ => unreachable!(),
        };

        let b = match right {
            RuntimeValue::IntegerLiteral(n) => n as f64,
            RuntimeValue::FloatLiteral(n) => n,
            _ => unreachable!(),
        };

        let result = f(a, b);

        // If both operands were integers and result is whole, keep as integer
        if !left.is_float() && !right.is_float() && result.fract() == 0.0 {
            RuntimeValue::IntegerLiteral(result as i32)
        } else {
            RuntimeValue::FloatLiteral(result)
        }
    }

    /// String concatenation
    pub(crate) fn string_concat(
        &mut self,
        left: RuntimeValue,
        left_span: Span,
        right: RuntimeValue,
        right_span: Span,
    ) -> Result<(), HydorError> {
        let left_idx = match left {
            RuntimeValue::StringLiteral(v) => v,
            _ => unreachable!(),
        };

        let right_idx = match right {
            RuntimeValue::StringLiteral(v) => v,
            _ => unreachable!(),
        };

        let left_str = self.resolve_string(left_idx);
        let right_str = self.resolve_string(right_idx);

        let concatenated = format!("{}{}", left_str, right_str);

        // Intern the new string
        let str_index = self.intern_string(concatenated);

        let result_span = Span {
            line: left_span.line,
            start_column: left_span.start_column,
            end_column: right_span.end_column,
        };

        self.push(RuntimeValue::StringLiteral(str_index), result_span)?;
        Ok(())
    }
}
