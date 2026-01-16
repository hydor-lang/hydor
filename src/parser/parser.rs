use std::collections::HashMap;

use crate::{
    ast::{Expr, Expression, Program, Statement, Stmt},
    errors::{ErrorCollector, HydorError},
    parser::lookups::Precedence,
    tokens::{Token, TokenInfo, TokenType},
    utils::Spanned,
};

type PrefixParseFn = fn(&mut Parser) -> Option<Expression>;
type InfixParseFn = fn(&mut Parser, Expression) -> Option<Expression>;
type StatementParseFn = fn(&mut Parser) -> Option<Statement>;

pub struct Parser {
    tokens: Vec<TokenInfo>,
    current: usize,
    delimiter_stack: Vec<TokenType>,

    pub led_parse_fns: HashMap<TokenType, InfixParseFn>,
    pub nud_parse_fns: HashMap<TokenType, PrefixParseFn>,
    pub stmt_parse_fns: HashMap<TokenType, StatementParseFn>,

    pub errors: ErrorCollector,
}

impl Parser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        let mut parser = Self {
            tokens,
            current: 0,
            errors: ErrorCollector::new(),
            delimiter_stack: Vec::new(),

            led_parse_fns: HashMap::new(),
            nud_parse_fns: HashMap::new(),
            stmt_parse_fns: HashMap::new(),
        };

        parser.register_nud(TokenType::Integer, Parser::parse_integer_literal);
        parser.register_nud(TokenType::Float, Parser::parse_float_literal);
        parser.register_nud(TokenType::False, Parser::parse_bool_literal);
        parser.register_nud(TokenType::True, Parser::parse_bool_literal);
        parser.register_nud(TokenType::Identifier, Parser::parse_identifier_literal);
        parser.register_nud(TokenType::String, Parser::parse_string_literal);
        parser.register_nud(TokenType::Nil, Parser::parse_nil_literal);

        parser.register_nud(TokenType::Minus, Parser::parse_unary_expr);
        parser.register_nud(TokenType::Not, Parser::parse_unary_expr);
        parser.register_nud(TokenType::LeftParenthesis, Parser::parse_grouping_expr);

        parser.register_led(TokenType::Plus, Parser::parse_binary_expr);
        parser.register_led(TokenType::Minus, Parser::parse_binary_expr);
        parser.register_led(TokenType::Asterisk, Parser::parse_binary_expr);
        parser.register_led(TokenType::Slash, Parser::parse_binary_expr);
        parser.register_led(TokenType::Caret, Parser::parse_exponent_expr);

        parser.register_led(TokenType::LessThan, Parser::parse_binary_expr);
        parser.register_led(TokenType::LessThanEqual, Parser::parse_binary_expr);
        parser.register_led(TokenType::GreaterThan, Parser::parse_binary_expr);
        parser.register_led(TokenType::GreaterThanEqual, Parser::parse_binary_expr);
        parser.register_led(TokenType::Equal, Parser::parse_binary_expr);
        parser.register_led(TokenType::NotEqual, Parser::parse_binary_expr);

        parser
    }

    pub fn parse_program(&mut self) -> Result<Program, ErrorCollector> {
        let mut body: Vec<Statement> = Vec::new();

        while !self.is_eof() {
            match self.try_parse_statement() {
                Some(stmt) => body.push(stmt),
                None => {
                    // Synchronize: skip to next statement boundary
                    self.synchronize();
                }
            }
        }

        if self.errors.has_errors() {
            Err(std::mem::take(&mut self.errors))
        } else {
            Ok(Program { statements: body })
        }
    }

    fn register_nud(&mut self, token: TokenType, func: PrefixParseFn) {
        self.nud_parse_fns.insert(token, func);
    }

    fn register_led(&mut self, token: TokenType, func: InfixParseFn) {
        self.led_parse_fns.insert(token, func);
    }

    fn register_stmt(&mut self, token: TokenType, func: StatementParseFn) {
        self.stmt_parse_fns.insert(token, func);
    }

    fn advance(&mut self) {
        if self.current < self.tokens.len() - 1 {
            self.current += 1;
        }
    }

    fn is_at_delimiter(&self) -> bool {
        matches!(
            self.current_token().token.get_type(),
            TokenType::Semicolon | TokenType::Newline
        )
    }

    fn expect(&mut self, token_type: TokenType) -> bool {
        if self.current_token().token.get_type() != token_type {
            let expect_err_msg = HydorError::ExpectedToken {
                expected: token_type,
                got: self.current_token().token.get_type(),
                span: self.current_token().span,
            };

            self.errors.add(expect_err_msg);
            return false;
        }

        self.current += 1;
        true
    }

    fn expect_delimiter(&mut self) -> bool {
        // If we're inside delimiters, delimiters are optional
        if !self.delimiter_stack.is_empty() {
            // Just skip any delimiters if present, but don't require them
            while self.is_at_delimiter() && !self.is_eof() {
                self.advance();
            }
            return true; // Always succeed when inside delimiters
        }

        // Outside delimiters - require a delimiter
        match self.current_token().token.get_type() {
            TokenType::EndOfFile => true,
            TokenType::Semicolon | TokenType::Newline => {
                // Consume all consecutive delimiters
                while self.is_at_delimiter() && !self.is_eof() {
                    self.advance();
                }
                true
            }
            _ => {
                let expect_err_msg = HydorError::ExpectedToken {
                    expected: TokenType::Semicolon,
                    got: self.current_token().token.get_type(),
                    span: self.current_token().span,
                };

                self.errors.add(expect_err_msg);
                false
            }
        }
    }

    fn skip_newlines_in_delimiters(&mut self) {
        if !self.delimiter_stack.is_empty() {
            while matches!(
                self.current_token().token.get_type(),
                TokenType::Newline | TokenType::Semicolon
            ) && !self.is_eof()
            {
                self.advance();
            }
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_eof() {
            // Stop at statement boundaries
            if self.is_at_delimiter() {
                self.advance(); // consume the delimiter
                return;
            }

            // Stop before keywords that start new statements
            // (when you add them: let, fn, if, while, etc.)

            self.advance();
        }
    }

    fn current_token(&self) -> &TokenInfo {
        self.tokens
            .get(self.current)
            .unwrap_or_else(|| self.tokens.last().expect("Token vector is empty!"))
    }

    fn is_eof(&self) -> bool {
        self.current_token().token == Token::EndOfFile || self.current >= self.tokens.len()
    }

    pub fn try_parse_expression(&mut self, precedence: u8) -> Option<Expression> {
        self.skip_newlines_in_delimiters();

        let token_type = self.current_token().token.get_type();

        // Get prefix parser function
        let prefix_fn = match self.nud_parse_fns.get(&token_type) {
            Some(f) => *f,
            None => {
                self.errors.add(HydorError::UnexpectedToken {
                    token: token_type,
                    span: self.current_token().span,
                });
                return None;
            }
        };

        let mut left = prefix_fn(self)?;

        // Parse infix expressions
        while !self.is_eof() {
            self.skip_newlines_in_delimiters();

            let token_type = self.current_token().token.get_type();
            let next_prec =
                Precedence::get_token_precedence(&token_type).unwrap_or(Precedence::Default);
            let next_prec_value = next_prec.into();

            if precedence >= next_prec_value {
                break;
            }

            let infix_fn = match self.led_parse_fns.get(&token_type) {
                Some(f) => *f,
                None => break,
            };

            left = infix_fn(self, left)?;
        }

        Some(left)
    }

    fn try_parse_statement(&mut self) -> Option<Statement> {
        let stmt_type = self.current_token().token.get_type();

        // Try to parse as a statement
        if let Some(stmt_fn) = self.stmt_parse_fns.get(&stmt_type) {
            return stmt_fn(self);
        }

        // expression statement
        let start = self.current_token().clone();

        let expr = self.try_parse_expression(Precedence::Default.into())?;

        self.expect_delimiter();

        Some(Spanned {
            node: Stmt::Expression { expression: expr },
            span: start.span,
        })
    }
}

// ------------------- EXPRESSIONS -------------------

impl Parser {
    // ------------------- Null Denoted Expressions -------------------
    pub fn parse_integer_literal(&mut self) -> Option<Expression> {
        let token_info = self.current_token();
        let value = match token_info.token {
            Token::Integer(n) => n,
            _ => unreachable!(),
        };

        let expr = Expr::IntegerLiteral(value).spanned(token_info.span);

        self.advance();
        Some(expr)
    }

    pub fn parse_float_literal(&mut self) -> Option<Expression> {
        let token_info = self.current_token();
        let value = match token_info.token {
            Token::Float(n) => n,
            _ => unreachable!(),
        };

        let expr = Expr::FloatLiteral(value).spanned(token_info.span);

        self.advance();
        Some(expr)
    }

    pub fn parse_bool_literal(&mut self) -> Option<Expression> {
        let token_info = self.current_token();
        let value = match token_info.token {
            Token::True => true,
            Token::False => false,
            _ => unreachable!(),
        };

        let expr = Expr::BooleanLiteral(value).spanned(token_info.span);

        self.advance();
        Some(expr)
    }

    pub fn parse_identifier_literal(&mut self) -> Option<Expression> {
        let token_info = self.current_token();
        let ident = match token_info.token.clone() {
            Token::Identifier(name) => name,
            _ => unreachable!(),
        };

        let expr = Expr::Identifier(ident).spanned(token_info.span);

        self.advance();
        Some(expr)
    }

    pub fn parse_string_literal(&mut self) -> Option<Expression> {
        let token_info = self.current_token();
        let ident = match token_info.token.clone() {
            Token::String(name) => name,
            _ => unreachable!(),
        };

        let expr = Expr::StringLiteral(ident).spanned(token_info.span);

        self.advance();
        Some(expr)
    }

    pub fn parse_nil_literal(&mut self) -> Option<Expression> {
        let token_info = self.current_token();
        let expr = Expr::NilLiteral.spanned(token_info.span);

        self.advance(); // Eat token
        Some(expr)
    }

    pub fn parse_unary_expr(&mut self) -> Option<Expression> {
        let operator_info = self.current_token().clone();
        self.advance(); // Eat operator

        let value = self.try_parse_expression(Precedence::Unary.into())?;
        let expr = Expr::Unary {
            operator: operator_info.token,
            right: Box::new(value),
        }
        .spanned(operator_info.span);

        Some(expr)
    }

    pub fn parse_grouping_expr(&mut self) -> Option<Expression> {
        self.advance(); // Eat '('
        self.delimiter_stack.push(TokenType::LeftParenthesis);

        // skip \n after (
        self.skip_newlines_in_delimiters();
        let expr = self.try_parse_expression(Precedence::Default.into())?;
        self.skip_newlines_in_delimiters();

        self.delimiter_stack.pop(); // Remove (

        if !self.expect(TokenType::RightParenthesis) {
            return None;
        }

        Some(expr)
    }

    // ------------------- Left Denoted Expressions -------------------
    pub fn parse_binary_expr(&mut self, left: Expression) -> Option<Expression> {
        let operator_info = self.current_token().clone();
        let operator_precedence =
            match Precedence::get_token_precedence(&operator_info.token.get_type()) {
                Some(p) => p,
                _ => Precedence::Default,
            };

        self.advance(); // Eat operator

        let right = self.try_parse_expression(operator_precedence.into())?;
        let expr = Expr::BinaryOperation {
            left: Box::new(left),
            operator: operator_info.token.clone(),
            right: Box::new(right),
        }
        .spanned(operator_info.span);

        Some(expr)
    }

    pub fn parse_exponent_expr(&mut self, left: Expression) -> Option<Expression> {
        let operator_info = self.current_token().clone();
        let operator_precedence: u8 =
            match Precedence::get_token_precedence(&operator_info.token.get_type()) {
                Some(p) => p,
                _ => Precedence::Default,
            }
            .into();

        self.advance(); // Eat operator

        // Parse right-associative
        let right = self.try_parse_expression(operator_precedence - 1)?;
        let expr = Expr::BinaryOperation {
            left: Box::new(left),
            operator: operator_info.token.clone(),
            right: Box::new(right),
        }
        .spanned(operator_info.span);

        Some(expr)
    }
}
