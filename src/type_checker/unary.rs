use std::result;

use crate::{
    ast::Expression,
    errors::HydorError,
    tokens::{Token, TokenType},
    type_checker::type_checker::{Type, TypeChecker},
    utils::Span,
};

impl TypeChecker {
    pub(crate) fn check_unary(
        &mut self,
        operator: &Token,
        right: &Expression,
        span: Span,
    ) -> Option<Type> {
        let right_type = self.check_expression(right)?;

        match operator.get_token_type() {
            TokenType::Not => {
                if right_type != Type::Bool {
                    self.throw_error(HydorError::InvalidUnaryOp {
                        operator: "not".to_string(),
                        operand_type: right_type,
                        span,
                    });
                    return None;
                }
                Some(Type::Bool)
            }

            TokenType::Minus => {
                if right_type != Type::Integer && right_type != Type::Float {
                    self.throw_error(HydorError::InvalidUnaryOp {
                        operator: "-".to_string(),
                        operand_type: right_type,
                        span,
                    });
                    return None;
                }
                Some(right_type)
            }

            _ => unreachable!("Unknown unary operator"),
        }
    }
}
