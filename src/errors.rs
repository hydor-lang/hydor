use crate::{
    ast::Node, runtime_value::RuntimeType, tokens::TokenType, type_checker::type_checker::Type,
    utils::Span,
};
use colored::*;

#[derive(Debug, Clone)]
pub enum HydorError {
    // ----- Parser -----
    UnexpectedToken {
        token: TokenType,
        span: Span,
    },
    ExpectedToken {
        expected: TokenType,
        got: TokenType,
        span: Span,
    },
    ExpectedIdentifier {
        span: Span,
    },

    // ----- Type Checker -----
    UndefinedVariable {
        name: String,
        span: Span,
    },
    TypeMismatch {
        expected: Type,
        found: Type,
        span: Span,
    },
    InvalidUnaryOp {
        operator: String,
        operand_type: Type,
        span: Span,
    },
    InvalidBinaryOp {
        operator: String,
        left_type: Type,
        right_type: Type,
        span: Span,
    },

    // ----- Compiler -----
    UnknownAST {
        node: Node,
        span: Span,
    },

    // ----- HydorVM -----
    StackUnderflow {
        stack_length: usize,
        span: Span,
    },
    StackOverflow {
        stack_length: usize,
        span: Span,
    },

    // ----- Runtime Arithmetic Errors -----
    ArithmeticError {
        operation: String,
        left_type: RuntimeType,
        right_type: RuntimeType,
        span: Span,
    },
    UnaryOperationError {
        operation: String,
        operand_type: RuntimeType,
        span: Span,
    },
    ComparisonOperationError {
        operation: String,
        blame_type: RuntimeType,
        span: Span,
    },
}

impl HydorError {
    pub fn span(&self) -> Span {
        match self {
            HydorError::UnexpectedToken { span, .. } => *span,
            HydorError::ExpectedToken { span, .. } => *span,
            HydorError::ExpectedIdentifier { span } => *span,

            HydorError::UndefinedVariable { span, .. } => *span,
            HydorError::TypeMismatch { span, .. } => *span,
            HydorError::InvalidUnaryOp { span, .. } => *span,
            HydorError::InvalidBinaryOp { span, .. } => *span,

            HydorError::UnknownAST { span, .. } => *span,

            HydorError::StackUnderflow { span, .. } => *span,
            HydorError::StackOverflow { span, .. } => *span,
            HydorError::ArithmeticError { span, .. } => *span,
            HydorError::UnaryOperationError { span, .. } => *span,
            HydorError::ComparisonOperationError { span, .. } => *span,
        }
    }

    pub fn category(&self) -> &str {
        match self {
            HydorError::UnexpectedToken { .. } => "Syntax",
            HydorError::ExpectedToken { .. } => "Syntax",
            HydorError::ExpectedIdentifier { .. } => "Syntax",

            HydorError::UndefinedVariable { .. } => "Type",
            HydorError::TypeMismatch { .. } => "Type",
            HydorError::InvalidUnaryOp { .. } => "Type",
            HydorError::InvalidBinaryOp { .. } => "Type",

            HydorError::UnknownAST { .. } => "Compiler",

            HydorError::StackUnderflow { .. } => "Runtime",
            HydorError::StackOverflow { .. } => "Runtime",
            HydorError::ArithmeticError { .. } => "Runtime",
            HydorError::UnaryOperationError { .. } => "Runtime",
            HydorError::ComparisonOperationError { .. } => "Runtime",
        }
    }

    pub fn message(&self) -> String {
        match self {
            HydorError::UnexpectedToken { token, .. } => {
                format!("Unexpected token '{:?}'", token)
            }
            HydorError::ExpectedToken { expected, got, .. } => {
                format!("Expected '{:?}', but found '{:?}'", expected, got)
            }
            HydorError::ExpectedIdentifier { .. } => "Expected an identifier".to_string(),

            HydorError::UndefinedVariable { name, .. } => {
                format!("Variable '{}' is not defined", name)
            }
            HydorError::TypeMismatch {
                expected, found, ..
            } => {
                format!("Type mismatch: expected {:?}, found {:?}", expected, found)
            }
            HydorError::InvalidUnaryOp {
                operator,
                operand_type,
                ..
            } => {
                format!("Cannot apply '{}' to {:?}", operator, operand_type)
            }
            HydorError::InvalidBinaryOp {
                operator,
                left_type,
                right_type,
                ..
            } => {
                if left_type == right_type {
                    format!("Cannot use '{}' on {:?}", operator, left_type)
                } else {
                    format!(
                        "Cannot use '{}' between {:?} and {:?}",
                        operator, left_type, right_type
                    )
                }
            }

            HydorError::UnknownAST { node, .. } => match node {
                Node::Statement(s) => {
                    format!("Cannot compile statement\n\n{:#?}", s)
                }
                Node::Expression(e) => {
                    format!("Cannot compile expression\n\n{:#?}", e)
                }
            },

            HydorError::StackUnderflow { stack_length, .. } => {
                format!("Stack underflow (stack size: {})", stack_length)
            }
            HydorError::StackOverflow { stack_length, .. } => {
                format!("Stack overflow (stack size: {})", stack_length)
            }

            HydorError::ArithmeticError {
                operation,
                left_type,
                right_type,
                ..
            } => {
                if left_type == right_type {
                    format!(
                        "Cannot perform {} on '{}'",
                        operation,
                        left_type.to_string()
                    )
                } else {
                    format!(
                        "Cannot perform {} between '{}' and '{}'",
                        operation,
                        left_type.to_string(),
                        right_type.to_string()
                    )
                }
            }

            HydorError::UnaryOperationError {
                operation,
                operand_type,
                ..
            } => {
                format!(
                    "Cannot perform {} on '{}'",
                    operation,
                    operand_type.to_string()
                )
            }

            HydorError::ComparisonOperationError {
                operation,
                blame_type,
                ..
            } => {
                format!("Cannot use '{}' on '{}'", operation, blame_type.to_string())
            }
        }
    }

    pub fn hint(&self) -> Option<String> {
        match self {
            HydorError::UnexpectedToken { token, .. } => {
                Some(format!("Try removing or replacing '{:?}'", token))
            }
            HydorError::ExpectedToken { expected, got, .. } => Some(format!(
                "Replace '{:?}' with '{:?}' or insert '{:?}' before it",
                got, expected, expected
            )),
            HydorError::ExpectedIdentifier { .. } => Some(
                "Variable names must be identifiers like 'x' or 'count', not expressions"
                    .to_string(),
            ),

            HydorError::UndefinedVariable { name, .. } => Some(format!(
                "Make sure '{}' is declared with 'let' before using it",
                name
            )),
            HydorError::TypeMismatch {
                expected, found, ..
            } => Some(format!(
                "Try converting {:?} to {:?}, or use a different operation",
                found, expected
            )),
            HydorError::InvalidUnaryOp {
                operator,
                operand_type,
                ..
            } => match operator.as_str() {
                "not" | "!" => Some(format!(
                    "The 'not' operator only works on Bool, not {:?}",
                    operand_type
                )),
                "-" => Some(format!(
                    "Negation only works on Int and Float, not {:?}",
                    operand_type
                )),
                _ => Some("This operator is not valid for this type".to_string()),
            },
            HydorError::InvalidBinaryOp {
                operator,
                left_type,
                right_type,
                ..
            } => {
                if left_type != right_type {
                    Some(format!(
                        "Both sides of '{}' must be the same type",
                        operator
                    ))
                } else {
                    match operator.as_str() {
                        "+" | "-" | "*" | "/" | "^" => Some(format!(
                            "Arithmetic operators only work on Int and Float, not {:?}",
                            left_type
                        )),
                        "<" | "<=" | ">" | ">=" => Some(format!(
                            "Comparison operators only work on Int and Float, not {:?}",
                            left_type
                        )),
                        _ => Some("Try using compatible types for this operation".to_string()),
                    }
                }
            }

            HydorError::UnknownAST { .. } => {
                Some("This AST node is not yet implemented in the compiler".to_string())
            }
            HydorError::StackUnderflow { .. } => {
                Some("This is likely a compiler bug - please report it".to_string())
            }
            HydorError::StackOverflow { .. } => Some(
                "Try simplifying your expression or splitting it into smaller parts".to_string(),
            ),

            HydorError::ArithmeticError {
                operation,
                left_type,
                right_type,
                ..
            } => {
                let valid_types = match operation.as_str() {
                    "addition" => "numbers or strings",
                    "subtraction" | "multiplication" | "division" | "exponentiation" => "numbers",
                    _ => "compatible types",
                };

                if left_type == right_type {
                    Some(format!(
                        "'{}' requires {}, but '{}' doesn't support it",
                        operation,
                        valid_types,
                        left_type.to_string()
                    ))
                } else {
                    Some(format!(
                        "'{}' requires both sides to be {}",
                        operation, valid_types
                    ))
                }
            }

            HydorError::UnaryOperationError {
                operation,
                operand_type,
                ..
            } => {
                let valid_type = match operation.as_str() {
                    "negation" => "a number (Int or Float)",
                    "logical not" => "a boolean (Bool)",
                    _ => "a compatible type",
                };

                Some(format!(
                    "'{}' requires {}, not '{}'",
                    operation,
                    valid_type,
                    operand_type.to_string()
                ))
            }

            HydorError::ComparisonOperationError { .. } => {
                Some("Comparisons only work on numbers (Int or Float)".to_string())
            }
        }
    }

    pub fn report(&self, source: &str) {
        let span = self.span();

        // Header: Category::Error -> message
        eprintln!(
            "{}{}{}{}",
            self.category().bright_white().bold(),
            "::".white().dimmed(),
            "Error".red().dimmed().bold(),
            format!(" -> {}", self.message()).bright_red()
        );

        eprintln!();

        // Error caused by section
        eprintln!("{}", "Error caused by:".white().dimmed().bold());

        let lines: Vec<&str> = source.lines().collect();

        if span.line > 0 && span.line <= (lines.len() as u32) {
            // Show error line with full info
            let line_content = lines[(span.line - 1) as usize];
            eprintln!(
                "    {} {} {}",
                format!("Ln {}:{}", span.line, span.start_column)
                    .cyan()
                    .bold(),
                "|".white(),
                line_content.bold().bright_white(),
            );

            // Calculate pointer position
            let line_prefix_len = format!("Ln {}:{}", span.line, span.start_column).len();
            let gutter_padding = " ".repeat(line_prefix_len + 3); // +3 for " | "
            let code_padding = " ".repeat(span.start_column.saturating_sub(1) as usize);

            let width = span.end_column.saturating_sub(span.start_column).max(1) as usize;

            let pointer = if width == 1 {
                "^".to_string()
            } else {
                "~".repeat(width)
            };

            eprintln!(
                "    {}{}{}",
                gutter_padding,
                code_padding,
                pointer.bright_red().bold()
            );
        } else {
            eprintln!(
                "    {} {} {}",
                format!("Ln {}:{}", span.line, span.start_column).cyan(),
                "|".white(),
                "<source unavailable>".dimmed()
            );
        }

        eprintln!();

        // Hint section
        if let Some(hint_text) = self.hint() {
            eprintln!("{} {}", "Hint:".bright_yellow(), hint_text.bright_white());
        }

        eprintln!();
    }
}

#[derive(Debug, Default)]
pub struct ErrorCollector {
    errors: Vec<HydorError>,
}

impl ErrorCollector {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add(&mut self, error: HydorError) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }

    pub fn report_all(&self, source: &str) {
        for error in &self.errors {
            error.report(source);
        }

        if !self.errors.is_empty() {
            let error_word = if self.errors.len() == 1 {
                "error"
            } else {
                "errors"
            };

            eprintln!(
                "{} Could not compile due to {} {}",
                "*".bright_red().bold(),
                self.errors.len().to_string().bright_red().bold(),
                error_word.bright_red()
            );
        }
    }

    pub fn errors(&self) -> &[HydorError] {
        &self.errors
    }

    pub fn clear(&mut self) {
        self.errors.clear()
    }
}
