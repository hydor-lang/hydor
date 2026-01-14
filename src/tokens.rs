use crate::utils::Span;

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

#[derive(Eq, PartialEq, Hash, Debug)]
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

impl TokenType {
    pub fn get_delimiters() -> Vec<TokenType> {
        vec![
            TokenType::EndOfFile,
            TokenType::Semicolon,
            TokenType::Newline,
        ]
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

    pub fn get_type(&self) -> TokenType {
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
