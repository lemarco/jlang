mod error;
pub use error::{ParseError, Result};

use crate::ast::*;
use crate::lexer::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program> {
        let mut program = Program::new();

        while !self.is_at_end() {
            if self.match_token(&TokenType::Module) {
                let module = self.parse_module()?;
                program.modules.push(module);
            } else {
                return Err(ParseError::UnexpectedToken {
                    expected: "module".to_string(),
                    found: format!("{:?}", self.peek().token_type),
                    line: self.peek().line,
                    column: self.peek().column,
                });
            }
        }

        Ok(program)
    }

    fn parse_module(&mut self) -> Result<Module> {
        // Parse module name
        let name = match &self.advance().token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: format!("{:?}", self.previous().token_type),
                    line: self.previous().line,
                    column: self.previous().column,
                });
            }
        };

        // Expect opening brace
        self.consume(&TokenType::LeftBrace, "Expected '{' after module name")?;

        let mut statements = Vec::new();

        // Parse statements until closing brace
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        // Consume the closing brace
        self.consume(&TokenType::RightBrace, "Expected '}' after module body")?;

        Ok(Module { name, statements })
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        if self.match_token(&TokenType::Let) {
            self.parse_let_statement()
        } else if self.match_token(&TokenType::Const) {
            self.parse_const_statement()
        } else if self.match_token(&TokenType::Type) {
            self.parse_type_definition()
        } else {
            Err(ParseError::UnexpectedToken {
                expected: "let, const, or type".to_string(),
                found: format!("{:?}", self.peek().token_type),
                line: self.peek().line,
                column: self.peek().column,
            })
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement> {
        let name = match &self.advance().token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: format!("{:?}", self.previous().token_type),
                    line: self.previous().line,
                    column: self.previous().column,
                });
            }
        };

        self.consume(&TokenType::Equals, "Expected '=' after variable name")?;

        let value = Box::new(self.parse_expression()?);

        Ok(Statement::Let { name, value })
    }

    fn parse_const_statement(&mut self) -> Result<Statement> {
        let name = match &self.advance().token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: format!("{:?}", self.previous().token_type),
                    line: self.previous().line,
                    column: self.previous().column,
                });
            }
        };

        self.consume(&TokenType::Equals, "Expected '=' after constant name")?;

        let value = Box::new(self.parse_expression()?);

        Ok(Statement::Const { name, value })
    }

    fn parse_type_definition(&mut self) -> Result<Statement> {
        let name = match &self.advance().token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: format!("{:?}", self.previous().token_type),
                    line: self.previous().line,
                    column: self.previous().column,
                });
            }
        };

        self.consume(&TokenType::Arrow, "Expected '=>' after type name")?;
        self.consume(&TokenType::LeftBrace, "Expected '{' after '=>'")?;

        let mut fields = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            fields.push(self.parse_type_field()?);
            if self.check(&TokenType::Comma) {
                self.advance();
            }
        }

        self.consume(&TokenType::RightBrace, "Expected '}' after type fields")?;

        Ok(Statement::TypeDef(TypeDefinition { name, fields }))
    }

    fn parse_type_field(&mut self) -> Result<TypeField> {
        let name = match &self.advance().token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: format!("{:?}", self.previous().token_type),
                    line: self.previous().line,
                    column: self.previous().column,
                });
            }
        };

        self.consume(&TokenType::Colon, "Expected ':' after field name")?;

        let field_type = self.parse_type()?;

        Ok(TypeField { name, field_type })
    }

    fn parse_type(&mut self) -> Result<Type> {
        match &self.advance().token_type {
            TokenType::Number => Ok(Type::Number),
            TokenType::String => Ok(Type::String),
            TokenType::Boolean => Ok(Type::Boolean),
            TokenType::Identifier(name) => Ok(Type::Custom(name.clone())),
            _ => Err(ParseError::UnexpectedToken {
                expected: "type".to_string(),
                found: format!("{:?}", self.previous().token_type),
                line: self.previous().line,
                column: self.previous().column,
            }),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        match &self.peek().token_type {
            TokenType::NumberLiteral(_)
            | TokenType::StringLiteral(_)
            | TokenType::BooleanLiteral(_)
            | TokenType::Identifier(_) => Ok(self.parse_primary()?),
            TokenType::LeftBrace => self.parse_object_expression(),
            _ => Err(ParseError::UnexpectedToken {
                expected: "expression".to_string(),
                found: format!("{:?}", self.peek().token_type),
                line: self.peek().line,
                column: self.peek().column,
            }),
        }
    }

    fn parse_primary(&mut self) -> Result<Expression> {
        let token = self.advance();
        match &token.token_type {
            TokenType::NumberLiteral(n) => Ok(Expression::NumberLiteral(*n)),
            TokenType::StringLiteral(s) => Ok(Expression::StringLiteral(s.clone())),
            TokenType::BooleanLiteral(b) => Ok(Expression::BooleanLiteral(*b)),
            TokenType::Identifier(name) => Ok(Expression::Identifier(name.clone())),
            _ => Err(ParseError::UnexpectedToken {
                expected: "literal or identifier".to_string(),
                found: format!("{:?}", token.token_type),
                line: token.line,
                column: token.column,
            }),
        }
    }

    fn parse_object_expression(&mut self) -> Result<Expression> {
        self.consume(&TokenType::LeftBrace, "Expected '{' for object literal")?;

        let mut fields = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let name = match &self.advance().token_type {
                TokenType::Identifier(name) => name.clone(),
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        expected: "identifier".to_string(),
                        found: format!("{:?}", self.previous().token_type),
                        line: self.previous().line,
                        column: self.previous().column,
                    });
                }
            };

            self.consume(&TokenType::Colon, "Expected ':' after field name")?;

            let value = self.parse_expression()?;
            fields.push((name, value));

            if self.check(&TokenType::Comma) {
                self.advance();
            }
        }

        self.consume(&TokenType::RightBrace, "Expected '}' after object fields")?;

        Ok(Expression::Object { fields })
    }

    // Helper methods
    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::EOF)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().token_type == token_type
        }
    }

    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", token_type),
                found: format!("{:?}", self.peek().token_type),
                line: self.peek().line,
                column: self.peek().column,
            })
        }
    }
}
