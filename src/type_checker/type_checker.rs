use crate::{
    ast::{Expr, Expression, Program, Statement, Stmt},
    errors::{ErrorCollector, HydorError},
};
use core::fmt;
use std::mem;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    Float,
    Bool,
    String,
    Nil,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Integer => write!(f, "Integer"),
            Type::Float => write!(f, "Float"),
            Type::Bool => write!(f, "Bool"),
            Type::String => write!(f, "String"),
            Type::Nil => write!(f, "Nil"),
        }
    }
}

pub struct TypeChecker {
    errors: ErrorCollector,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            errors: ErrorCollector::new(),
        }
    }

    /// Main entry point - call this before compilation
    pub fn check_program(&mut self, program: &Program) -> Result<(), ErrorCollector> {
        for stmt in &program.statements {
            let result = self.check_statement(stmt);

            if result.is_none() {
                break;
            }
        }

        if self.errors.has_errors() {
            Err(mem::take(&mut self.errors))
        } else {
            Ok(())
        }
    }

    pub(crate) fn check_statement(&mut self, stmt: &Statement) -> Option<()> {
        match &stmt.node {
            Stmt::Expression { expression } => {
                // Type check the expression, ignore result
                self.check_expression(expression)?;
            }

            _ => unimplemented!(),
        }

        Some(())
    }

    pub(crate) fn check_expression(&mut self, expr: &Expression) -> Option<Type> {
        let span = expr.span;

        match &expr.node {
            Expr::IntegerLiteral(_) => Some(Type::Integer),
            Expr::FloatLiteral(_) => Some(Type::Float),
            Expr::BooleanLiteral(_) => Some(Type::Bool),
            Expr::StringLiteral(_) => Some(Type::String),
            Expr::NilLiteral => Some(Type::Nil),

            Expr::Unary { operator, right } => {
                let unary_type = self.check_unary(operator, right, span)?;
                Some(unary_type)
            }

            Expr::BinaryOperation {
                left,
                operator,
                right,
            } => {
                let binary_expr_type = self.check_binary_expr(operator, left, right, span)?;
                Some(binary_expr_type)
            }
            _ => unreachable!("Unknown unary operator"),
        }
    }

    pub(crate) fn throw_error(&mut self, error: HydorError) {
        self.errors.add(error);
    }
}
