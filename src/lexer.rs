use crate::{
    tokens::{Token, TokenInfo},
    utils::Span,
};
pub struct Lexer {
    input: Vec<char>,
    position: usize, // current position
    line: usize,     // current line
    column: usize,   // current column
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn peek_char(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.current_char()?;
        self.position += ch.len_utf8();

        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            match ch {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn skip_comment(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn read_number(&mut self) -> Token {
        let mut number = String::new();
        let mut is_float = false;

        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                number.push(ch);
                self.advance();
            } else if ch == '.'
                && !is_float
                && self.peek_char().map_or(false, |c| c.is_ascii_digit())
            {
                is_float = true;
                number.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if is_float {
            Token::Float(number.parse().unwrap_or(0.0))
        } else {
            Token::Integer(number.parse().unwrap_or(0))
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();

        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        Token::lookup_identifier(&identifier)
    }

    fn read_string(&mut self) -> Token {
        self.advance(); // skip opening
        let mut string = String::new();

        while let Some(ch) = self.current_char() {
            if ch == '"' || ch == '\'' {
                self.advance(); // skip closing 
                return Token::String(string);
            }

            if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char() {
                    match escaped {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        'r' => string.push('\r'),
                        '"' => string.push('"'),
                        '\\' => string.push('\\'),
                        _ => {
                            string.push('\\');
                            string.push(escaped);
                        }
                    }
                    self.advance();
                }
            } else {
                string.push(ch);
                self.advance();
            }
        }

        // Unterminated string
        Token::Illegal('"')
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek_char() == Some(expected) {
            self.advance(); // consume the peeked character
            true
        } else {
            false
        }
    }

    pub fn next_token(&mut self) -> TokenInfo {
        self.skip_whitespace();

        let start_line = self.line;
        let start_column = self.column;

        let token = match self.current_char() {
            None => Token::EndOfFile,

            Some('\n') => {
                self.advance();
                Token::Newline
            }

            // Comments
            Some('/') if self.peek_char() == Some('/') => {
                self.skip_comment();
                return self.next_token(); // recursively get next token
            }

            // Single-character tokens
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
            Some(':') => {
                self.advance();
                if self.match_char(':') {
                    Token::BoxColon
                } else {
                    Token::Colon
                }
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

            // Two-character tokens
            Some('=') => {
                self.advance();
                if self.match_char('=') {
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            Some('!') => {
                self.advance();
                if self.match_char('=') {
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            Some('<') => {
                self.advance();
                if self.match_char('=') {
                    Token::LessThanEqual
                } else {
                    Token::LessThan
                }
            }
            Some('>') => {
                self.advance();
                if self.match_char('=') {
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

        let end_column = self.column - 1;

        TokenInfo {
            token,
            span: Span {
                line: start_line,
                start_column,
                end_column,
            },
        }
    }

    pub fn tokenize(&mut self) -> Vec<TokenInfo> {
        let mut tokens = Vec::new();

        loop {
            let token_info = self.next_token();
            let is_eof = token_info.token == Token::EndOfFile;
            tokens.push(token_info);

            if is_eof {
                break;
            }
        }

        tokens
    }
}
