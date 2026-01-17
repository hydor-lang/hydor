use crate::{
    ast::type_annotation::TypeAnnotation,
    tokens::Token,
    utils::{Span, Spanned},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

pub type Expression = Spanned<Expr>;
pub type Statement = Spanned<Stmt>;

// Used fot error handling
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Statement(Stmt),
    Expression(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    IntegerLiteral(i32),
    FloatLiteral(f64),
    BooleanLiteral(bool),
    StringLiteral(String),
    Identifier(String),
    NilLiteral,

    Unary {
        operator: Token,
        right: Box<Expression>,
    },

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
        annotated_type: TypeAnnotation,
        span: Span,
    },
}

impl Expr {
    pub fn spanned(self, span: Span) -> Spanned<Self> {
        Spanned { node: self, span }
    }

    pub fn to_node(self) -> Node {
        Node::Expression(self.clone())
    }
}

impl Stmt {
    pub fn spanned(self, span: Span) -> Spanned<Self> {
        Spanned { node: self, span }
    }

    pub fn to_node(self) -> Node {
        Node::Statement(self.clone())
    }
}
