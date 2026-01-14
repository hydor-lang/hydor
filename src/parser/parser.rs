use crate::{
    ast::{Expr, Expression, Program, Statement, Stmt},
    parser::lookups::Precedence,
    tokens::{Token, TokenInfo, TokenType},
    utils::Spanned,
};
use std::collections::HashMap;

type PrefixParseFn = fn(&mut Parser) -> Expression;
type InfixParseFn = fn(&mut Parser, Expression) -> Expression;
type StatementParseFn = fn(&mut Parser) -> Statement;

pub struct Parser {
    tokens: Vec<TokenInfo>,
    current: usize,
    pub led_parse_fns: HashMap<TokenType, InfixParseFn>,
    pub nud_parse_fns: HashMap<TokenType, PrefixParseFn>,
    pub stmt_parse_fns: HashMap<TokenType, StatementParseFn>,
}

impl Parser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        let mut parser = Self {
            tokens,
            current: 0,
            led_parse_fns: HashMap::new(),
            nud_parse_fns: HashMap::new(),
            stmt_parse_fns: HashMap::new(),
        };
        parser.register_nud(TokenType::Integer, Parser::parse_integer);
        parser
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
        if !self.is_eof() {
            self.current += 1;
        }
    }

    fn expect(&mut self, token_type: TokenType) -> bool {
        if self.current_token().token.get_type() != token_type {
            return false;
        }

        self.current += 1;
        true
    }

    fn expect_one(&mut self, token_types: Vec<TokenType>) -> bool {
        for tok_type in token_types {
            if self.current_token().token.get_type() == tok_type {
                self.current += 1;
                return true;
            }
        }

        false
    }

    fn current_token(&self) -> &TokenInfo {
        &self.tokens[self.current]
    }

    fn is_eof(&self) -> bool {
        self.current >= self.tokens.len()
    }

    pub fn parse_program(&mut self) -> Program {
        let mut body: Vec<Statement> = Vec::new();
        while !self.is_eof() {
            let stmt = self.parse_statement();
            body.push(stmt);
        }
        Program { statements: body }
    }

    fn parse_statement(&mut self) -> Statement {
        let stmt_type = self.current_token().token.get_type();
        let stmt_fn = match self.stmt_parse_fns.get(&stmt_type) {
            Some(f) => {
                return f(self);
            }

            _ => {
                let span = self.current_token().span;
                let expr = self.parse_expression(Precedence::Default);
                self.expect_one(TokenType::get_delimiters());

                let expr_stmt = Stmt::Expression { expression: expr };

                Spanned {
                    node: expr_stmt,
                    span,
                }
            }
        };

        stmt_fn
    }

    pub fn parse_expression(&mut self, precedence: Precedence) -> Expression {
        let token_type = self.current_token().token.get_type();
        let prefix_fn = match self.nud_parse_fns.get(&token_type) {
            Some(f) => *f,
            None => panic!("No prefix parse function for token {:?}", token_type),
        };

        let mut left = prefix_fn(self);

        while !self.is_eof() {
            let token_type = self.current_token().token.get_type();
            let next_prec =
                Precedence::get_token_precedence(&token_type).unwrap_or(Precedence::Default);
            if precedence >= next_prec {
                break;
            }

            let infix_fn = match self.led_parse_fns.get(&token_type) {
                Some(f) => *f,
                None => break,
            };

            left = infix_fn(self, left);
        }

        left
    }

    pub fn parse_integer(&mut self) -> Expression {
        let token_info = self.current_token();
        let value = match token_info.token {
            Token::Integer(n) => n,
            _ => unreachable!(),
        };
        let expr = Expr::IntegerLiteral(value);
        let spanned_expr = Spanned {
            node: expr,
            span: token_info.span.clone(),
        };
        self.advance();
        spanned_expr
    }
}
