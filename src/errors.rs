use crate::{ast::Node, tokens::TokenType, utils::Span};
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
}

impl HydorError {
    pub fn span(&self) -> Span {
        match self {
            HydorError::UnexpectedToken { span, .. } => *span,
            HydorError::ExpectedToken { span, .. } => *span,

            HydorError::UnknownAST { span, .. } => *span,
        }
    }

    pub fn category(&self) -> &str {
        match self {
            HydorError::UnexpectedToken { .. } => "Syntax",
            HydorError::ExpectedToken { .. } => "Syntax",

            HydorError::UnknownAST { .. } => "AST",
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
