use crate::tokens::TokenType;
use num_enum::IntoPrimitive;

#[derive(IntoPrimitive, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Precedence {
    Default,
    Assignment,
    Ternary,
    LogicalOr,
    LogicalAnd,
    Equals,
    Comparison,
    Additive,
    Multiplicative,
    Exponent,
    Unary,
    Call,
    Grouping,
}

impl Precedence {
    pub fn get_token_precedence(token_type: &TokenType) -> Option<Precedence> {
        match token_type {
            TokenType::Assign => Some(Precedence::Assignment),
            TokenType::If => Some(Precedence::Ternary),
            TokenType::Or => Some(Precedence::LogicalOr),
            TokenType::And => Some(Precedence::LogicalAnd),
            TokenType::Equal | TokenType::NotEqual => Some(Precedence::Equals),
            TokenType::LessThan
            | TokenType::LessThanEqual
            | TokenType::GreaterThan
            | TokenType::GreaterThanEqual => Some(Precedence::Comparison),
            TokenType::Plus | TokenType::Minus => Some(Precedence::Additive),
            TokenType::Asterisk | TokenType::Slash => Some(Precedence::Multiplicative),
            TokenType::Caret => Some(Precedence::Exponent),
            TokenType::LeftParenthesis | TokenType::BoxColon => Some(Precedence::Call),
            TokenType::Not => Some(Precedence::Unary),
            _ => None,
        }
    }
}
