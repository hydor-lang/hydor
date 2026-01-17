use crate::{
    ast::ast::Expression,
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
    ) -> Result<Type, ()> {
        let right_type = self.check_expression(right)?;
        let op_token = operator.get_token_type();

        match op_token {
            TokenType::Not => {
                if right_type != Type::Bool {
                    self.throw_error(HydorError::InvalidUnaryOp {
                        operator: op_token,
                        operand_type: right_type,
                        span,
                    });
                    return Err(());
                }
                Ok(Type::Bool)
            }

            TokenType::Minus => {
                if right_type != Type::Integer && right_type != Type::Float {
                    self.throw_error(HydorError::InvalidUnaryOp {
                        operator: op_token,
                        operand_type: right_type,
                        span,
                    });
                    return Err(());
                }
                Ok(right_type)
            }

            _ => unreachable!("Unknown unary operator"),
        }
    }
}
