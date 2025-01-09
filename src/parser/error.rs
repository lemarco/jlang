use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken {
        expected: String,
        found: String,
        line: usize,
        column: usize,
    },
    InvalidExpression {
        message: String,
        line: usize,
        column: usize,
    },
    UnexpectedEOF {
        line: usize,
        column: usize,
    },
    // Add more error types as needed
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken {
                expected,
                found,
                line,
                column,
            } => write!(
                f,
                "Unexpected token at line {}, column {}. Expected {}, found {}",
                line, column, expected, found
            ),
            ParseError::InvalidExpression {
                message,
                line,
                column,
            } => write!(
                f,
                "Invalid expression at line {}, column {}: {}",
                line, column, message
            ),
            ParseError::UnexpectedEOF { line, column } => write!(
                f,
                "Unexpected end of file at line {}, column {}",
                line, column
            ),
        }
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;
