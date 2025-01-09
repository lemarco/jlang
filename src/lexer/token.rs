// src/lexer/token.rs

/// Represents the different types of tokens in the language
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    Module,
    Type,
    Const,
    Let,

    // Types
    Number,
    String,
    Boolean,

    // Symbols
    LeftBrace,  // {
    RightBrace, // }
    LeftParen,  // (
    RightParen, // )
    Colon,      // :
    Arrow,      // =>
    Equals,     // =
    Dot,        // .
    Comma,      // ,

    // Values
    Identifier(String),
    NumberLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),

    EOF,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Module => write!(f, "module"),
            TokenType::Type => write!(f, "type"),
            TokenType::Const => write!(f, "const"),
            TokenType::Let => write!(f, "let"),
            TokenType::Number => write!(f, "Number"),
            TokenType::String => write!(f, "String"),
            TokenType::Boolean => write!(f, "Boolean"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::Colon => write!(f, ":"),
            TokenType::Arrow => write!(f, "=>"),
            TokenType::Equals => write!(f, "="),
            TokenType::Dot => write!(f, "."),
            TokenType::Comma => write!(f, ","),
            TokenType::Identifier(s) => write!(f, "{}", s),
            TokenType::NumberLiteral(n) => write!(f, "{}", n),
            TokenType::StringLiteral(s) => write!(f, "\"{}\"", s),
            TokenType::BooleanLiteral(b) => write!(f, "{}", b),
            TokenType::EOF => write!(f, "EOF"),
        }
    }
}

/// Represents a token with its type and position information
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    /// Creates a new token with the given type and position
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Token {
            token_type,
            line,
            column,
        }
    }

    /// Returns true if this token is of the given type
    pub fn is_type(&self, token_type: TokenType) -> bool {
        self.token_type == token_type
    }

    /// Returns true if this token represents a literal value
    pub fn is_literal(&self) -> bool {
        matches!(
            self.token_type,
            TokenType::NumberLiteral(_)
                | TokenType::StringLiteral(_)
                | TokenType::BooleanLiteral(_)
        )
    }

    /// Returns true if this token represents a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(
            self.token_type,
            TokenType::Module | TokenType::Type | TokenType::Const | TokenType::Let
        )
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at line {}, column {}",
            self.token_type, self.line, self.column
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_display() {
        let token = Token::new(TokenType::Identifier("test".to_string()), 1, 1);
        assert_eq!(token.to_string(), "test at line 1, column 1");

        let token = Token::new(TokenType::NumberLiteral(42.0), 2, 3);
        assert_eq!(token.to_string(), "42 at line 2, column 3");

        let token = Token::new(TokenType::StringLiteral("hello".to_string()), 3, 4);
        assert_eq!(token.to_string(), "\"hello\" at line 3, column 4");
    }

    #[test]
    fn test_token_type_checks() {
        let token = Token::new(TokenType::Module, 1, 1);
        assert!(token.is_keyword());
        assert!(!token.is_literal());

        let token = Token::new(TokenType::NumberLiteral(42.0), 1, 1);
        assert!(token.is_literal());
        assert!(!token.is_keyword());

        let token = Token::new(TokenType::Identifier("test".to_string()), 1, 1);
        assert!(!token.is_keyword());
        assert!(!token.is_literal());
    }
}
