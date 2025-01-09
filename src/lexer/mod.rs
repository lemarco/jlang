mod error;
mod token;

pub use error::LexerError;
pub use token::{Token, TokenType};

/// Lexer for tokenizing source code.
/// Tracks position and handles error reporting with line and column information.
#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Creates a new Lexer instance from input string
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    /// Returns true if we've reached the end of input
    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }

    /// Returns the current character without consuming it
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.current]
        }
    }

    /// Returns the next character without consuming it
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.input.len() {
            '\0'
        } else {
            self.input[self.current + 1]
        }
    }

    /// Consumes and returns the current character
    fn advance(&mut self) -> Result<char, LexerError> {
        if self.is_at_end() {
            return Err(LexerError::UnexpectedEOF {
                line: self.line,
                column: self.column,
            });
        }
        let c = self.input[self.current];
        self.current += 1;
        self.column += 1;
        Ok(c)
    }

    /// Conditionally consumes the next character if it matches expected
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.input[self.current] != expected {
            false
        } else {
            self.current += 1;
            self.column += 1;
            true
        }
    }

    /// Skips whitespace and comments, updating line and column numbers
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance().unwrap();
                }
                '\n' => {
                    self.line += 1;
                    self.column = 1;
                    self.advance().unwrap();
                }
                '/' if self.peek_next() == '/' => {
                    // Skip comments until end of line
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance().unwrap();
                    }
                }
                _ => break,
            }
        }
        self.start = self.current;
    }

    /// Creates a token of the given type at current position
    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            line: self.line,
            column: self.column - (self.current - self.start),
        }
    }

    /// Handles string literals
    fn string(&mut self) -> Result<Token, LexerError> {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            self.advance()?;
        }

        if self.is_at_end() {
            return Err(LexerError::UnterminatedString {
                line: self.line,
                column: self.column,
            });
        }

        // Consume the closing quote
        self.advance()?;

        // Get string content (excluding quotes)
        let content: String = self.input[self.start + 1..self.current - 1]
            .iter()
            .collect();

        Ok(self.make_token(TokenType::StringLiteral(content)))
    }

    /// Handles number literals (both integer and float)
    fn number(&mut self) -> Result<Token, LexerError> {
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            self.advance()?;
        }

        // Look for decimal point
        if !self.is_at_end() && self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the dot
            self.advance()?;

            while !self.is_at_end() && self.peek().is_ascii_digit() {
                self.advance()?;
            }
        }

        let num_str: String = self.input[self.start..self.current].iter().collect();

        match num_str.parse::<f64>() {
            Ok(value) => Ok(self.make_token(TokenType::NumberLiteral(value))),
            Err(_) => Err(LexerError::InvalidNumber {
                line: self.line,
                column: self.column - num_str.len(),
            }),
        }
    }

    /// Handles identifiers and keywords
    fn identifier(&mut self) -> Result<Token, LexerError> {
        while !self.is_at_end() && (self.peek().is_ascii_alphanumeric() || self.peek() == '_') {
            self.advance()?;
        }

        let text: String = self.input[self.start..self.current].iter().collect();

        let token_type = match text.as_str() {
            "module" => TokenType::Module,
            "type" => TokenType::Type,
            "const" => TokenType::Const,
            "let" => TokenType::Let,
            "Number" => TokenType::Number,
            "String" => TokenType::String,
            "Boolean" => TokenType::Boolean,
            "true" => TokenType::BooleanLiteral(true),
            "false" => TokenType::BooleanLiteral(false),
            _ => TokenType::Identifier(text),
        };

        Ok(self.make_token(token_type))
    }

    /// Returns the next token in the input
    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();

        if self.is_at_end() {
            return Ok(self.make_token(TokenType::EOF));
        }

        self.start = self.current;
        let c = self.advance()?;

        match c {
            '{' => Ok(self.make_token(TokenType::LeftBrace)),
            '}' => Ok(self.make_token(TokenType::RightBrace)),
            '(' => Ok(self.make_token(TokenType::LeftParen)),
            ')' => Ok(self.make_token(TokenType::RightParen)),
            ':' => Ok(self.make_token(TokenType::Colon)),
            '=' => {
                if self.match_char('>') {
                    Ok(self.make_token(TokenType::Arrow))
                } else {
                    Ok(self.make_token(TokenType::Equals))
                }
            }
            '.' => Ok(self.make_token(TokenType::Dot)),
            ',' => Ok(self.make_token(TokenType::Comma)),
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if c.is_ascii_alphabetic() || c == '_' => self.identifier(),
            _ => Err(LexerError::UnexpectedCharacter {
                char: c,
                line: self.line,
                column: self.column - 1,
            }),
        }
    }

    /// Consumes all tokens and returns them as a vector
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            let is_eof = token.token_type == TokenType::EOF;
            tokens.push(token);

            if is_eof {
                break;
            }
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("");
        assert!(matches!(
            lexer.next_token().unwrap().token_type,
            TokenType::EOF
        ));
    }

    #[test]
    fn test_single_character_tokens() {
        let mut lexer = Lexer::new("{}():.,");
        let expected = vec![
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::Colon,
            TokenType::Dot,
            TokenType::Comma,
            TokenType::EOF,
        ];

        for expected_type in expected {
            assert_eq!(lexer.next_token().unwrap().token_type, expected_type);
        }
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("module type const let");
        let expected = vec![
            TokenType::Module,
            TokenType::Type,
            TokenType::Const,
            TokenType::Let,
            TokenType::EOF,
        ];

        for expected_type in expected {
            assert_eq!(lexer.next_token().unwrap().token_type, expected_type);
        }
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::new(r#""Hello, World!""#);
        match lexer.next_token().unwrap().token_type {
            TokenType::StringLiteral(s) => assert_eq!(s, "Hello, World!"),
            _ => panic!("Expected string literal"),
        }
    }

    #[test]
    fn test_number_literal() {
        let mut lexer = Lexer::new("42 3.14");

        match lexer.next_token().unwrap().token_type {
            TokenType::NumberLiteral(n) => assert_eq!(n, 42.0),
            _ => panic!("Expected number literal"),
        }

        match lexer.next_token().unwrap().token_type {
            TokenType::NumberLiteral(n) => assert_eq!(n, 3.14),
            _ => panic!("Expected number literal"),
        }
    }

    #[test]
    fn test_identifier() {
        let mut lexer = Lexer::new("foo_bar123");
        match lexer.next_token().unwrap().token_type {
            TokenType::Identifier(s) => assert_eq!(s, "foo_bar123"),
            _ => panic!("Expected identifier"),
        }
    }

    #[test]
    fn test_unterminated_string() {
        let mut lexer = Lexer::new("\"unterminated");
        assert!(matches!(
            lexer.next_token().unwrap_err(),
            LexerError::UnterminatedString { .. }
        ));
    }

    #[test]
    fn test_comments() {
        let mut lexer = Lexer::new("let x = 42 // This is a comment\nlet y = 23");
        let expected = vec![
            TokenType::Let,
            TokenType::Identifier("x".to_string()),
            TokenType::Equals,
            TokenType::NumberLiteral(42.0),
            TokenType::Let,
            TokenType::Identifier("y".to_string()),
            TokenType::Equals,
            TokenType::NumberLiteral(23.0),
            TokenType::EOF,
        ];

        for expected_type in expected {
            assert_eq!(lexer.next_token().unwrap().token_type, expected_type);
        }
    }
}
