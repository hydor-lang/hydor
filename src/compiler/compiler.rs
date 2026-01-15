use std::mem;

use crate::{
    ast::{Expr, Expression, Node, Program, Statement, Stmt},
    bytecode::bytecode::{Instructions, OpCode},
    errors::{ErrorCollector, HydorError},
    runtime_value::RuntimeValue,
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
            match self.try_compile_node(stmt.as_node()) {
                None => break,
                Some(()) => (),
            };
        }

        // Emit halt with a dummy span (or track program end span)
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

    fn try_compile_node(&mut self, node: Node) -> Option<()> {
        match node {
            Node::Statement(stmt) => self.try_compile_statement(stmt)?,
            Node::Expression(expr) => self.try_compile_expression(expr)?,
        }

        Some(()) // Success
    }

    fn try_compile_statement(&mut self, stmt: Statement) -> Option<()> {
        let span = stmt.span;

        match stmt.node {
            Stmt::Expression { expression } => self.try_compile_expression(expression)?,
            unknown => {
                self.throw_error(HydorError::UnknownAST {
                    node: Node::Statement(Spanned {
                        node: unknown,
                        span,
                    }),
                    span,
                });
                return None;
            }
        }

        Some(()) // Success
    }

    fn try_compile_expression(&mut self, expr: Expression) -> Option<()> {
        let span = expr.span;

        match expr.node {
            Expr::IntegerLiteral(v) => {
                let value = RuntimeValue::IntegerLiteral(v);
                let constant_index = self.add_constant(value);

                self.emit(OpCode::LoadConstant, vec![constant_index], span);
            }

            Expr::FloatLiteral(v) => {
                let value = RuntimeValue::FloatLiteral(v);
                let constant_index = self.add_constant(value);
                self.emit(OpCode::LoadConstant, vec![constant_index], span);
            }

            Expr::BooleanLiteral(v) => {
                let value = RuntimeValue::BooleanLiteral(v);
                let constant_index = self.add_constant(value);
                self.emit(OpCode::LoadConstant, vec![constant_index], span);
            }

            Expr::StringLiteral(v) => {
                let value = RuntimeValue::StringLiteral(v);
                let constant_index = self.add_constant(value);
                self.emit(OpCode::LoadConstant, vec![constant_index], span);
            }

            unknown => {
                self.throw_error(HydorError::UnknownAST {
                    node: Node::Expression(Spanned {
                        node: unknown,
                        span,
                    }),
                    span,
                });
                return None;
            }
        }

        Some(()) // Success
    }

    fn bytecode(&mut self) -> Bytecode {
        Bytecode {
            instructions: mem::take(&mut self.instructions),
            constants: mem::take(&mut self.constants),
            debug_info: mem::take(&mut self.debug_info),
        }
    }

    /// Emit an instruction with span tracking (RLE compression happens here)
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
            if self.should_add_line_change(offset, span.line) {
                self.debug_info.line_changes.push((offset, span.line));
            }

            if self.should_add_col_change(
                &self.debug_info.start_col_changes,
                offset,
                span.start_column,
            ) {
                self.debug_info
                    .start_col_changes
                    .push((offset, span.start_column));
            }

            if self.should_add_col_change(&self.debug_info.end_col_changes, offset, span.end_column)
            {
                self.debug_info
                    .end_col_changes
                    .push((offset, span.end_column));
            }

            self.instructions.push(byte);
        }

        position
    }

    /// Check if we need to record a line change (for compression)
    fn should_add_line_change(&self, offset: usize, line: u32) -> bool {
        self.debug_info.line_changes.is_empty()
            || self.debug_info.line_changes.last().unwrap().1 != line
    }

    /// Check if we need to record a column change (for compression)
    fn should_add_col_change(&self, changes: &Vec<(usize, u32)>, offset: usize, col: u32) -> bool {
        changes.is_empty() || changes.last().unwrap().1 != col
    }
}
