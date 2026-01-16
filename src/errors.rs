use crate::{ast::Node, runtime_value::RuntimeType, tokens::TokenType, utils::Span};
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

    // ----- Arithmetic Errors -----
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

            HydorError::UnknownAST { .. } => "AST",

            HydorError::StackUnderflow { .. } => "HydorVM",
            HydorError::StackOverflow { .. } => "HydorVM",
            HydorError::ArithmeticError { .. } => "Arithmetic",
            HydorError::UnaryOperationError { .. } => "Arithmetic",
            HydorError::ComparisonOperationError { .. } => "Boolean",
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
            HydorError::UnknownAST { node, .. } => match node {
                Node::Statement(s) => {
                    format!("Cannot compile AST statement\n\n{:#?}", s)
                }
                Node::Expression(e) => {
                    format!("Cannot compile AST expression\n\n{:#?}", e)
                }
            },

            HydorError::StackUnderflow { stack_length, .. } => {
                format!("Stack underflow! stack length: {}", stack_length)
            }
            HydorError::StackOverflow { stack_length, .. } => {
                format!("Stack overflow! stack length: {}", stack_length)
            }

            HydorError::ArithmeticError {
                operation,
                left_type,
                right_type,
                ..
            } => {
                if left_type == right_type {
                    format!(
                        "Cannot perform {} on type '{}'",
                        operation,
                        left_type.to_string()
                    )
                } else {
                    format!(
                        "Cannot perform {} between types '{}' and '{}'",
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
                    "Cannot perform {} on type '{}'",
                    operation,
                    operand_type.to_string()
                )
            }

            HydorError::ComparisonOperationError {
                operation,
                blame_type,
                ..
            } => {
                format!(
                    "Cannot compare with '{}' on type '{}'",
                    operation,
                    blame_type.to_string()
                )
            }
        }
    }

    pub fn hint(&self) -> Option<String> {
        match self {
            HydorError::UnexpectedToken { token, .. } => {
                Some(format!("Try removing or replacing '{:?}'", token))
            }
            HydorError::ExpectedToken { expected, got, .. } => Some(format!(
                "Try replacing '{:?}' with '{:?}' or insert '{:?}' before '{:?}'",
                got, expected, expected, got
            )),
            HydorError::UnknownAST { .. } => {
                Some(format!("Try defining a compiler for the given ast node"))
            }
            HydorError::StackUnderflow { .. } => None,
            HydorError::StackOverflow { .. } => None,

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
                        "The '{}' operation requires {}, but '{}' is not supported",
                        operation,
                        valid_types,
                        left_type.to_string()
                    ))
                } else {
                    Some(format!(
                        "The '{}' operation requires {}, ensure both operands are the same compatible type",
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
                    "negation" => "a number",
                    "logical not" => "a boolean",
                    _ => "a compatible type",
                };

                Some(format!(
                    "The '{}' operation requires {}, but got '{}'",
                    operation,
                    valid_type,
                    operand_type.to_string()
                ))
            }

            HydorError::ComparisonOperationError { .. } => {
                Some(format!("Try converting the operand's type to a number"))
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
