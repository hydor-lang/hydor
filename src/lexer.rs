use crate::{
    tokens::{Token, TokenInfo},
    utils::Span,
};

pub struct Lexer {
    input: Vec<char>,
    position: u32,
    line: u32,
    column: u32,
    last_token: Option<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
            last_token: None,
        }
    }

    fn current(&self) -> Option<char> {
        self.input.get(self.position as usize).copied()
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.input.get((self.position as usize) + offset).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.current()?;
        self.position += 1;

        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.current(), Some(' ' | '\t' | '\r')) {
            self.advance();
        }
    }

    fn skip_comment(&mut self) {
        self.advance(); // first /
        self.advance(); // second /

        while let Some(ch) = self.current() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn read_number(&mut self) -> Token {
        let mut num = String::new();
        let mut is_float = false;

        while let Some(ch) = self.current() {
            if ch.is_ascii_digit() {
                num.push(ch);
                self.advance();
            } else if ch == '.' && !is_float && self.peek(1).map_or(false, |c| c.is_ascii_digit()) {
                is_float = true;
                num.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if is_float {
            Token::Float(num.parse().unwrap_or(0.0))
        } else {
            Token::Integer(num.parse().unwrap_or(0))
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut id = String::new();

        while let Some(ch) = self.current() {
            if ch.is_alphanumeric() || ch == '_' {
                id.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        Token::lookup_identifier(&id)
    }

    fn read_string(&mut self) -> Token {
        self.advance(); // skip opening quote
        let mut s = String::new();

        while let Some(ch) = self.current() {
            if ch == '"' || ch == '\'' {
                self.advance(); // skip closing quote
                return Token::String(s);
            }

            if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.current() {
                    match escaped {
                        'n' => s.push('\n'),
                        't' => s.push('\t'),
                        'r' => s.push('\r'),
                        '"' => s.push('"'),
                        '\'' => s.push('\''),
                        '\\' => s.push('\\'),
                        _ => {
                            s.push('\\');
                            s.push(escaped);
                        }
                    }
                    self.advance();
                }
            } else {
                s.push(ch);
                self.advance();
            }
        }

        Token::Illegal('"')
    }

    pub fn next_token(&mut self) -> TokenInfo {
        self.skip_whitespace();

        // Handle comments
        if self.current() == Some('/') && self.peek(1) == Some('/') {
            self.skip_comment();
            return self.next_token();
        }

        let start_line = self.line;
        let start_col = self.column;

        let token = match self.current() {
            None => Token::EndOfFile,

            Some('\n') => {
                // Skip consecutive newlines
                if matches!(self.last_token, Some(Token::Newline)) {
                    self.advance();
                    return self.next_token();
                }
                self.advance();
                Token::Newline
            }

            // Single-char tokens
            Some('(') => {
                self.advance();
                Token::LeftParenthesis
            }
            Some(')') => {
                self.advance();
                Token::RightParenthesis
            }
            Some('{') => {
                self.advance();
                Token::LeftBrace
            }
            Some('}') => {
                self.advance();
                Token::RightBrace
            }
            Some('[') => {
                self.advance();
                Token::LeftBracket
            }
            Some(']') => {
                self.advance();
                Token::RightBracket
            }
            Some('.') => {
                self.advance();
                Token::Dot
            }
            Some(',') => {
                self.advance();
                Token::Comma
            }
            Some(';') => {
                self.advance();
                Token::Semicolon
            }
            Some('+') => {
                self.advance();
                Token::Plus
            }
            Some('-') => {
                self.advance();
                Token::Minus
            }
            Some('*') => {
                self.advance();
                Token::Asterisk
            }
            Some('/') => {
                self.advance();
                Token::Slash
            }
            Some('^') => {
                self.advance();
                Token::Caret
            }

            // Two-char tokens
            Some(':') => {
                self.advance();
                if self.current() == Some(':') {
                    self.advance();
                    Token::BoxColon
                } else {
                    Token::Colon
                }
            }

            Some('=') => {
                self.advance();
                if self.current() == Some('=') {
                    self.advance();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }

            Some('!') => {
                self.advance();
                if self.current() == Some('=') {
                    self.advance();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }

            Some('<') => {
                self.advance();
                if self.current() == Some('=') {
                    self.advance();
                    Token::LessThanEqual
                } else {
                    Token::LessThan
                }
            }

            Some('>') => {
                self.advance();
                if self.current() == Some('=') {
                    self.advance();
                    Token::GreaterThanEqual
                } else {
                    Token::GreaterThan
                }
            }

            // String literals
            Some('"') | Some('\'') => self.read_string(),

            // Numbers
            Some(ch) if ch.is_ascii_digit() => self.read_number(),

            // Identifiers and keywords
            Some(ch) if ch.is_alphabetic() || ch == '_' => self.read_identifier(),

            // Illegal character
            Some(ch) => {
                self.advance();
                Token::Illegal(ch)
            }
        };

        let end_col = self.column;

        self.last_token = Some(token.clone());

        let span = Span {
            line: start_line,
            start_column: start_col,
            end_column: end_col,
        };

        TokenInfo { token, span }
    }

    pub fn tokenize(&mut self) -> Vec<TokenInfo> {
        let mut tokens = Vec::new();

        loop {
            let info = self.next_token();
            let is_eof = info.token == Token::EndOfFile;

            // Skip leading newlines
            if tokens.is_empty() && info.token == Token::Newline {
                continue;
            }

            tokens.push(info);

            if is_eof {
                break;
            }
        }

        tokens
    }
}
