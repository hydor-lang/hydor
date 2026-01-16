use std::mem;

use crate::{
    ast::{Expr, Expression, Node, Program, Statement, Stmt},
    bytecode::bytecode::{Instructions, OpCode},
    errors::{ErrorCollector, HydorError},
    runtime_value::RuntimeValue,
    tokens::TokenType,
    utils::{Span, Spanned},
};

pub struct Compiler {
    instructions: Instructions,
    constants: Vec<RuntimeValue>,
    debug_info: DebugInfo,

    errors: ErrorCollector,
}

pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<RuntimeValue>,
    pub debug_info: DebugInfo,
}

/// Run-length encoded debug information
/// Stores only when line/column values change to save space
#[derive(Default)]
pub struct DebugInfo {
    /// (bytecode_offset, line_number) - stores line changes only
    pub line_changes: Vec<(usize, u32)>,

    /// (bytecode_offset, start_col) - stores start column changes only
    pub start_col_changes: Vec<(usize, u32)>,

    /// (bytecode_offset, end_col) - stores end column changes only
    pub end_col_changes: Vec<(usize, u32)>,
}

impl DebugInfo {
    pub fn new() -> Self {
        Self {
            line_changes: Vec::new(),
            start_col_changes: Vec::new(),
            end_col_changes: Vec::new(),
        }
    }

    /// Decompress: lookup the span for a given bytecode offset
    pub fn get_span(&self, ip: usize) -> Span {
        let line = self.find_value(&self.line_changes, ip);
        let start_column = self.find_value(&self.start_col_changes, ip);
        let end_column = self.find_value(&self.end_col_changes, ip);

        Span {
            line,
            start_column,
            end_column,
        }
    }

    /// Binary search to find the last value before or at the given offset
    fn find_value(&self, changes: &Vec<(usize, u32)>, ip: usize) -> u32 {
        if changes.is_empty() {
            return 0; // fallback
        }

        let idx = changes
            .binary_search_by_key(&ip, |&(offset, _)| offset)
            .unwrap_or_else(|i| i.saturating_sub(1));

        changes[idx].1
    }
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            debug_info: DebugInfo::new(),

            errors: ErrorCollector::new(),
        }
    }

    /// Main entry point
    pub fn compile_program(&mut self, program: Program) -> Result<Bytecode, ErrorCollector> {
        for stmt in program.statements {
            if self.try_compile_statement(stmt).is_none() {
                break;
            }
        }

        // Emit with dummy span
        self.emit(
            OpCode::Halt,
            vec![],
            Span {
                line: 0,
                start_column: 0,
                end_column: 0,
            },
        );

        if self.errors.has_errors() {
            Err(mem::take(&mut self.errors))
        } else {
            Ok(self.bytecode())
        }
    }

    fn try_compile_statement(&mut self, stmt: Statement) -> Option<()> {
        let span = stmt.span;

        match stmt.node {
            Stmt::Expression { expression } => {
                self.compile_expression(expression)?;
                self.emit(OpCode::Pop, vec![], span);
            }

            unknown => {
                self.throw_error(HydorError::UnknownAST {
                    node: unknown.to_node(),
                    span,
                });
                return None;
            }
        }

        Some(())
    }

    fn compile_expression(&mut self, expr: Expression) -> Option<()> {
        let span = expr.span;

        match expr.node {
            Expr::IntegerLiteral(v) => {
                let idx = self.add_constant(RuntimeValue::IntegerLiteral(v));
                self.emit(OpCode::LoadConstant, vec![idx], span);
            }

            Expr::FloatLiteral(v) => {
                let idx = self.add_constant(RuntimeValue::FloatLiteral(v));
                self.emit(OpCode::LoadConstant, vec![idx], span);
            }

            Expr::BooleanLiteral(v) => {
                let idx = self.add_constant(RuntimeValue::BooleanLiteral(v));
                self.emit(OpCode::LoadConstant, vec![idx], span);
            }

            Expr::StringLiteral(v) => {
                let idx = self.add_constant(RuntimeValue::StringLiteral(v));
                self.emit(OpCode::LoadConstant, vec![idx], span);
            }

            Expr::Unary { operator, right } => {
                self.compile_expression(*right);

                match operator.get_type() {
                    TokenType::Not => self.emit(OpCode::UnaryNot, vec![], span),
                    TokenType::Minus => self.emit(OpCode::UnaryNegate, vec![], span),

                    _ => unreachable!("Unhandled unary operator type"),
                };
            }

            Expr::BinaryOperation {
                left,
                operator,
                right,
            } => {
                self.compile_expression(*left);
                self.compile_expression(*right);

                match operator.get_type() {
                    TokenType::Plus => self.emit(OpCode::Add, vec![], span),
                    TokenType::Minus => self.emit(OpCode::Subtract, vec![], span),
                    TokenType::Asterisk => self.emit(OpCode::Subtract, vec![], span),
                    TokenType::Slash => self.emit(OpCode::Divide, vec![], span),
                    TokenType::Caret => self.emit(OpCode::Exponent, vec![], span),

                    _ => unreachable!("Unhandled binary operator type"),
                };
            }

            unknown => {
                self.throw_error(HydorError::UnknownAST {
                    node: unknown.to_node(),
                    span,
                });
                return None;
            }
        }

        Some(())
    }

    fn bytecode(&mut self) -> Bytecode {
        Bytecode {
            instructions: mem::take(&mut self.instructions),
            constants: mem::take(&mut self.constants),
            debug_info: mem::take(&mut self.debug_info),
        }
    }

    /// Emit an instruction with span tracking
    fn emit(&mut self, opcode: OpCode, operands: Vec<usize>, span: Span) -> usize {
        let instruction = OpCode::make(opcode, operands);
        let position = self.add_instruction(instruction, span);

        position
    }

    fn add_constant(&mut self, value: RuntimeValue) -> usize {
        self.constants.push(value);
        self.constants.len() - 1 // Returns the constant's "address"
    }

    fn throw_error(&mut self, error: HydorError) {
        self.errors.add(error);
    }

    /// Add instruction bytes and track their span n
    fn add_instruction(&mut self, instruction: Instructions, span: Span) -> usize {
        let position = self.instructions.len();

        // For each byte in the instruction, add span info (compressed)
        for byte in instruction {
            let offset = self.instructions.len();

            // Only add if values changed from last entry
            if self.should_add_line_change(span.line) {
                self.debug_info.line_changes.push((offset, span.line));
            }

            if self.should_add_col_change(&self.debug_info.start_col_changes, span.start_column) {
                self.debug_info
                    .start_col_changes
                    .push((offset, span.start_column));
            }

            if self.should_add_col_change(&self.debug_info.end_col_changes, span.end_column) {
                self.debug_info
                    .end_col_changes
                    .push((offset, span.end_column));
            }

            self.instructions.push(byte);
        }

        position
    }

    /// Check if we need to record a line change (for compression)
    fn should_add_line_change(&self, line: u32) -> bool {
        self.debug_info.line_changes.is_empty()
            || self.debug_info.line_changes.last().unwrap().1 != line
    }

    /// Check if we need to record a column change (for compression)
    fn should_add_col_change(&self, changes: &Vec<(usize, u32)>, col: u32) -> bool {
        changes.is_empty() || changes.last().unwrap().1 != col
    }
}
