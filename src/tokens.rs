use crate::utils::Span;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenInfo {
    pub token: Token,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Integer(i32),
    Float(f64),
    String(String),
    Identifier(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Caret,
    Bang,

    // Comparison
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    LessThanEqual,
    GreaterThanEqual,

    // Delimiters
    Comma,
    Semicolon,
    Colon,
    Newline,
    Dot,
    BoxColon, // ::

    // Grouping
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    // Special
    EndOfFile,
    Illegal(char),

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Nil,
    Or,
    And,
    Not,
    Return,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum TokenType {
    // Literals
    Integer,
    Float,
    String,
    Identifier,

    // Operators
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Caret,
    Bang,

    // Comparison
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    LessThanEqual,
    GreaterThanEqual,

    // Delimiters
    Comma,
    Semicolon,
    Colon,
    Newline,
    Dot,
    BoxColon, // ::

    // Grouping
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    // Special
    EndOfFile,
    Illegal,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Nil,
    Or,
    And,
    Not,
    Return,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Literals
            TokenType::Integer => write!(f, "Integer"),
            TokenType::Float => write!(f, "Float"),
            TokenType::String => write!(f, "String"),
            TokenType::Identifier => write!(f, "Identifier"),

            // Operators
            TokenType::Assign => write!(f, "="),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Asterisk => write!(f, "*"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Caret => write!(f, "^"),
            TokenType::Bang => write!(f, "!"),

            // Comparison
            TokenType::LessThan => write!(f, "<"),
            TokenType::GreaterThan => write!(f, ">"),
            TokenType::Equal => write!(f, "=="),
            TokenType::NotEqual => write!(f, "!="),
            TokenType::LessThanEqual => write!(f, "<="),
            TokenType::GreaterThanEqual => write!(f, ">="),

            // Delimiters
            TokenType::Comma => write!(f, ","),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Colon => write!(f, ":"),
            TokenType::Newline => write!(f, "newline"),
            TokenType::Dot => write!(f, "."),
            TokenType::BoxColon => write!(f, "::"),

            // Grouping
            TokenType::LeftParenthesis => write!(f, "("),
            TokenType::RightParenthesis => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::LeftBracket => write!(f, "["),
            TokenType::RightBracket => write!(f, "]"),

            // Special
            TokenType::EndOfFile => write!(f, "EOF"),
            TokenType::Illegal => write!(f, "illegal"),

            // Keywords
            TokenType::Function => write!(f, "fn"),
            TokenType::Let => write!(f, "let"),
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),
            TokenType::If => write!(f, "if"),
            TokenType::Else => write!(f, "else"),
            TokenType::Nil => write!(f, "nil"),
            TokenType::Or => write!(f, "or"),
            TokenType::And => write!(f, "and"),
            TokenType::Not => write!(f, "not"),
            TokenType::Return => write!(f, "return"),
        }
    }
}

impl Token {
    pub fn lookup_identifier(identifier: &str) -> Token {
        match identifier {
            "fn" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "nil" => Token::Nil,
            "or" => Token::Or,
            "and" => Token::And,
            "not" => Token::Not,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Identifier(identifier.to_string()),
        }
    }

    pub fn get_token_type(&self) -> TokenType {
        match self {
            // Literals
            Token::Integer(_) => TokenType::Integer,
            Token::Float(_) => TokenType::Float,
            Token::String(_) => TokenType::String,
            Token::Identifier(_) => TokenType::Identifier,

            // Operators
            Token::Assign => TokenType::Assign,
            Token::Plus => TokenType::Plus,
            Token::Minus => TokenType::Minus,
            Token::Asterisk => TokenType::Asterisk,
            Token::Slash => TokenType::Slash,
            Token::Caret => TokenType::Caret,
            Token::Bang => TokenType::Bang,

            // Comparison
            Token::LessThan => TokenType::LessThan,
            Token::GreaterThan => TokenType::GreaterThan,
            Token::Equal => TokenType::Equal,
            Token::NotEqual => TokenType::NotEqual,
            Token::LessThanEqual => TokenType::LessThanEqual,
            Token::GreaterThanEqual => TokenType::GreaterThanEqual,

            // Delimiters
            Token::Comma => TokenType::Comma,
            Token::Semicolon => TokenType::Semicolon,
            Token::Colon => TokenType::Colon,
            Token::Newline => TokenType::Newline,
            Token::Dot => TokenType::Dot,
            Token::BoxColon => TokenType::BoxColon,

            // Grouping
            Token::LeftParenthesis => TokenType::LeftParenthesis,
            Token::RightParenthesis => TokenType::RightParenthesis,
            Token::LeftBrace => TokenType::LeftBrace,
            Token::RightBrace => TokenType::RightBrace,
            Token::LeftBracket => TokenType::LeftBracket,
            Token::RightBracket => TokenType::RightBracket,

            // Special
            Token::EndOfFile => TokenType::EndOfFile,
            Token::Illegal(_) => TokenType::Illegal,

            // Keywords
            Token::Function => TokenType::Function,
            Token::Let => TokenType::Let,
            Token::True => TokenType::True,
            Token::False => TokenType::False,
            Token::If => TokenType::If,
            Token::Else => TokenType::Else,
            Token::Nil => TokenType::Nil,
            Token::Or => TokenType::Or,
            Token::And => TokenType::And,
            Token::Not => TokenType::Not,
            Token::Return => TokenType::Return,
        }
    }
}
