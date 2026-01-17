use crate::{
    ast::ast::Expression,
    errors::HydorError,
    tokens::{Token, TokenType},
    type_checker::type_checker::{Type, TypeChecker},
    utils::Span,
};

impl TypeChecker {
    pub(crate) fn check_binary_expr(
        &mut self,
        operator: &Token,
        left: &Expression,
        right: &Expression,
        span: Span,
    ) -> Result<Type, ()> {
        // If either side has an error, propagate it (stops cascading errors!)
        let left_type = self.check_expression(left)?;
        let right_type = self.check_expression(right)?;
        let op_tok = operator.get_token_type();

        match op_tok {
            // Arithmetic
            TokenType::Plus => {
                if left_type != right_type {
                    self.throw_error(HydorError::InvalidBinaryOp {
                        operator: op_tok,
                        left_type,
                        right_type,
                        span,
                    });
                    return Err(());
                }

                // Both must be numeric or string
                if left_type == Type::Integer
                    || left_type == Type::Float
                    || left_type == Type::String
                {
                    return Ok(left_type);
                }

                self.throw_error(HydorError::InvalidBinaryOp {
                    operator: op_tok,
                    left_type,
                    right_type,
                    span,
                });
                Err(())
            }

            TokenType::Minus | TokenType::Asterisk | TokenType::Slash | TokenType::Caret => {
                self.require_numeric_types(op_tok, left_type, right_type, span)
            }

            // Comparison - returns Bool, not the operand type!
            TokenType::LessThan
            | TokenType::LessThanEqual
            | TokenType::GreaterThan
            | TokenType::GreaterThanEqual => {
                self.require_numeric_types(op_tok, left_type, right_type, span)?;
                Ok(Type::Bool)
            }

            // Equality
            TokenType::Equal | TokenType::NotEqual => {
                if left_type != right_type {
                    self.throw_error(HydorError::InvalidBinaryOp {
                        operator: op_tok,
                        left_type,
                        right_type,
                        span,
                    });
                    return Err(());
                }

                Ok(Type::Bool)
            }

            _ => unreachable!("Unknown binary operator"),
        }
    }

    fn require_numeric_types(
        &mut self,
        op: TokenType,
        left: Type,
        right: Type,
        span: Span,
    ) -> Result<Type, ()> {
        if left != right {
            self.throw_error(HydorError::InvalidBinaryOp {
                operator: op,
                left_type: left,
                right_type: right,
                span,
            });
            return Err(());
        }

        if left != Type::Integer && left != Type::Float {
            self.throw_error(HydorError::InvalidBinaryOp {
                operator: op,
                left_type: left.clone(),
                right_type: right,
                span,
            });
            return Err(());
        }

        Ok(left)
    }
}
