use crate::{tokens::Token, utils::Spanned};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

pub type Expression = Spanned<Expr>;
pub type Statement = Spanned<Stmt>;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    IntegerLiteral(i32),
    FloatLiteral(f32),
    BooleanLiteral(bool),
    Identifier(String),
    StringLiteral(String),

    BinaryOperation {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression {
        expression: Expression,
    },
    VariableDeclaration {
        identifier: Expression,
        value: Expression,
    },
}
