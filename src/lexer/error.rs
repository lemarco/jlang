use std::fmt;

/// Represents possible errors that can occur during lexical analysis
#[derive(Debug, Clone, PartialEq)]
pub enum LexerError {
    /// String literal was not properly terminated
    UnterminatedString { line: usize, column: usize },

    /// Failed to parse a number literal
    InvalidNumber { line: usize, column: usize },

    /// Encountered an unexpected character
    UnexpectedCharacter {
        char: char,
        line: usize,
        column: usize,
    },

    /// Reached end of file unexpectedly
    UnexpectedEOF { line: usize, column: usize },
}

impl std::error::Error for LexerError {}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnterminatedString { line, column } => write!(
                f,
                "Unterminated string literal at line {}, column {}",
                line, column
            ),
            LexerError::InvalidNumber { line, column } => {
                write!(f, "Invalid number at line {}, column {}", line, column)
            }
            LexerError::UnexpectedCharacter { char, line, column } => write!(
                f,
                "Unexpected character '{}' at line {}, column {}",
                char, line, column
            ),
            LexerError::UnexpectedEOF { line, column } => write!(
                f,
                "Unexpected end of file at line {}, column {}",
                line, column
            ),
        }
    }
}

/// Result type for lexer operations
pub type Result<T> = std::result::Result<T, LexerError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = LexerError::UnterminatedString {
            line: 1,
            column: 10,
        };
        assert_eq!(
            err.to_string(),
            "Unterminated string literal at line 1, column 10"
        );

        let err = LexerError::InvalidNumber { line: 2, column: 5 };
        assert_eq!(err.to_string(), "Invalid number at line 2, column 5");

        let err = LexerError::UnexpectedCharacter {
            char: '@',
            line: 3,
            column: 15,
        };
        assert_eq!(
            err.to_string(),
            "Unexpected character '@' at line 3, column 15"
        );
    }

    #[test]
    fn test_error_debug() {
        let err = LexerError::UnexpectedEOF {
            line: 5,
            column: 20,
        };
        assert_eq!(
            format!("{:?}", err),
            "UnexpectedEOF { line: 5, column: 20 }"
        );
    }

    #[test]
    fn test_error_equality() {
        let err1 = LexerError::UnterminatedString {
            line: 1,
            column: 10,
        };
        let err2 = LexerError::UnterminatedString {
            line: 1,
            column: 10,
        };
        let err3 = LexerError::UnterminatedString {
            line: 2,
            column: 10,
        };

        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }
}
