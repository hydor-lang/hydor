use crate::{
    ast::Expression,
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
    ) -> Option<Type> {
        let left_type = self.check_expression(left)?;
        let right_type = self.check_expression(right)?;

        match operator.get_token_type() {
            // Arithmetic
            TokenType::Plus => {
                if left_type != right_type {
                    self.throw_error(HydorError::InvalidBinaryOp {
                        operator: operator.get_token_type().to_string(),
                        left_type,
                        right_type,
                        span,
                    });
                    return None;
                }

                // Both must be numeric
                if left_type == Type::Integer || left_type == Type::Float {
                    return Some(left_type);
                }

                // String concat
                if left_type == Type::String {
                    return Some(left_type);
                }

                self.throw_error(HydorError::InvalidBinaryOp {
                    operator: operator.get_token_type().to_string(),
                    left_type,
                    right_type,
                    span,
                });
                return None;
            }

            TokenType::Minus | TokenType::Asterisk | TokenType::Slash | TokenType::Caret => {
                if left_type != right_type {
                    self.throw_error(HydorError::InvalidBinaryOp {
                        operator: operator.get_token_type().to_string(),
                        left_type,
                        right_type,
                        span,
                    });
                    return None;
                }

                // Both must be numeric
                if left_type != Type::Integer && left_type != Type::Float {
                    self.throw_error(HydorError::InvalidBinaryOp {
                        operator: operator.get_token_type().to_string(),
                        left_type: left_type.clone(),
                        right_type,
                        span,
                    });
                    return None;
                }

                Some(left_type) // Result is same type as operands
            }

            // Comparison
            TokenType::LessThan
            | TokenType::LessThanEqual
            | TokenType::GreaterThan
            | TokenType::GreaterThanEqual => {
                if left_type != right_type {
                    self.throw_error(HydorError::InvalidBinaryOp {
                        operator: operator.get_token_type().to_string(),
                        left_type,
                        right_type,
                        span,
                    });
                    return None;
                }

                if left_type != Type::Integer && left_type != Type::Float {
                    self.throw_error(HydorError::InvalidBinaryOp {
                        operator: operator.get_token_type().to_string(),
                        left_type: left_type.clone(),
                        right_type,
                        span,
                    });
                    return None;
                }

                Some(Type::Bool)
            }

            // Equality
            TokenType::Equal | TokenType::NotEqual => {
                if left_type != right_type {
                    self.throw_error(HydorError::InvalidBinaryOp {
                        operator: operator.get_token_type().to_string(),
                        left_type,
                        right_type,
                        span,
                    });
                    return None;
                }

                Some(Type::Bool)
            }

            _ => unreachable!("Unknown binary operator"),
        }
    }
}
