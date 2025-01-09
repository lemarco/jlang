use jlang::*;

#[test]
fn test_parse_basic_module() {
    let source = r#"
        module test {
            let x = 42
        }
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);

    let program = parser.parse().unwrap();

    assert_eq!(program.modules.len(), 1);
    let module = &program.modules[0];
    assert_eq!(module.name, "test");
    assert_eq!(module.statements.len(), 1);

    match &module.statements[0] {
        Statement::Let { name, value } => {
            assert_eq!(name, "x");
            match **value {
                Expression::NumberLiteral(n) => assert_eq!(n, 42.0),
                _ => panic!("Expected number literal"),
            }
        }
        _ => panic!("Expected let statement"),
    }
}

#[test]
fn test_parse_type_definition() {
    let source = r#"
        module types {
            type Point => {
                x: Number,
                y: Number
            }
        }
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);

    let program = parser.parse().unwrap();

    assert_eq!(program.modules.len(), 1);
    let module = &program.modules[0];
    assert_eq!(module.name, "types");
    assert_eq!(module.statements.len(), 1);

    match &module.statements[0] {
        Statement::TypeDef(type_def) => {
            assert_eq!(type_def.name, "Point");
            assert_eq!(type_def.fields.len(), 2);
            assert_eq!(type_def.fields[0].name, "x");
            assert_eq!(type_def.fields[0].field_type, Type::Number);
            assert_eq!(type_def.fields[1].name, "y");
            assert_eq!(type_def.fields[1].field_type, Type::Number);
        }
        _ => panic!("Expected type definition"),
    }
}

#[test]
fn test_parse_const_declaration() {
    let source = r#"
        module constants {
            const PI = 3.14159
            const GREETING = "Hello"
            const ENABLED = true
        }
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);

    let program = parser.parse().unwrap();
    let module = &program.modules[0];

    assert_eq!(module.statements.len(), 3);

    match &module.statements[0] {
        Statement::Const { name, value } => {
            assert_eq!(name, "PI");
            match **value {
                Expression::NumberLiteral(n) => assert!((n - 3.14159).abs() < 1e-5),
                _ => panic!("Expected number literal"),
            }
        }
        _ => panic!("Expected const statement"),
    }

    match &module.statements[1] {
        Statement::Const { name, value } => {
            assert_eq!(name, "GREETING");
            match **value {
                Expression::StringLiteral(ref s) => assert_eq!(s, "Hello"),
                _ => panic!("Expected string literal"),
            }
        }
        _ => panic!("Expected const statement"),
    }

    match &module.statements[2] {
        Statement::Const { name, value } => {
            assert_eq!(name, "ENABLED");
            match **value {
                Expression::BooleanLiteral(b) => assert_eq!(b, true),
                _ => panic!("Expected boolean literal"),
            }
        }
        _ => panic!("Expected const statement"),
    }
}

#[test]
fn test_parse_object_literal() {
    let source = r#"
        module objects {
            let point = {
                x: 10,
                y: 20
            }
        }
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);

    let program = parser.parse().unwrap();
    let module = &program.modules[0];

    match &module.statements[0] {
        Statement::Let { name, value } => {
            assert_eq!(name, "point");
            match **value {
                Expression::Object { ref fields } => {
                    assert_eq!(fields.len(), 2);
                    assert_eq!(fields[0].0, "x");
                    match &fields[0].1 {
                        Expression::NumberLiteral(n) => assert_eq!(*n, 10.0),
                        _ => panic!("Expected number literal"),
                    }
                    assert_eq!(fields[1].0, "y");
                    match &fields[1].1 {
                        Expression::NumberLiteral(n) => assert_eq!(*n, 20.0),
                        _ => panic!("Expected number literal"),
                    }
                }
                _ => panic!("Expected object literal"),
            }
        }
        _ => panic!("Expected let statement"),
    }
}

#[test]
fn test_parse_errors() {
    // Test case 1: Missing module name
    let source = "module {";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Err(ParseError::UnexpectedToken {
            expected, found, ..
        }) => {
            assert!(expected.contains("identifier"));
        }
        other => panic!("Expected UnexpectedToken error, got {:?}", other),
    }

    // Test case 2: Missing opening brace
    let source = "module test }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Err(ParseError::UnexpectedToken {
            expected, found, ..
        }) => {
            assert!(expected.contains("LeftBrace") || expected.contains("{"));
        }
        other => panic!("Expected UnexpectedToken error, got {:?}", other),
    }

    // Test case 3: Missing closing brace
    let source = "module test { let x = 42";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Err(ParseError::UnexpectedToken {
            expected, found, ..
        }) => {
            assert!(expected.contains("RightBrace") || expected.contains("}"));
        }
        other => panic!("Expected UnexpectedToken error, got {:?}", other),
    }

    // Test case 4: Invalid token in type definition
    let source = "module test { type Point => { x: @ } }";
    let mut lexer = Lexer::new(source);
    match lexer.tokenize() {
        Err(_) => (), // Lexer should catch the invalid character
        Ok(tokens) => panic!("Expected lexer error for invalid character '@'"),
    }

    // Test case 5: Missing equals in let statement
    let source = "module test { let x 42 }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Err(ParseError::UnexpectedToken {
            expected, found, ..
        }) => {
            assert!(expected.contains("Equals") || expected.contains("="));
        }
        other => panic!("Expected UnexpectedToken error, got {:?}", other),
    }

    // Test case 6: Invalid type definition
    let source = "module test { type Point => let }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Err(ParseError::UnexpectedToken {
            expected, found, ..
        }) => {
            assert!(expected.contains("LeftBrace") || expected.contains("{"));
        }
        other => panic!("Expected UnexpectedToken error, got {:?}", other),
    }

    // Test case 7: Incomplete object literal
    let source = "module test { let point = { x: 10, }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Err(ParseError::UnexpectedToken {
            expected, found, ..
        }) => {
            assert!(expected.contains("RightBrace") || expected.contains("}"));
        }
        other => panic!("Expected UnexpectedToken error, got {:?}", other),
    }

    // Test case 8: Invalid type name
    let source = "module test { type 123 => { x: Number } }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Err(ParseError::UnexpectedToken {
            expected, found, ..
        }) => {
            assert!(expected.contains("identifier"));
        }
        other => panic!("Expected UnexpectedToken error, got {:?}", other),
    }
}

// Additional test to verify error position information

// Test for proper error messages
#[test]
fn test_error_messages() {
    let source = "module test { let x }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);

    match parser.parse() {
        Err(error) => {
            let error_string = format!("{}", error);
            assert!(
                error_string.contains("Expected"),
                "Error message should contain 'Expected': {}",
                error_string
            );
        }
        Ok(_) => panic!("Expected parser error"),
    }
}
