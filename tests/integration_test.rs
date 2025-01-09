use jlang::*;
use std::fs;

#[test]
fn test_basic_file() {
    let source = fs::read_to_string("tests/test_files/basic.j").unwrap();
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().unwrap();

    let expected = vec![
        TokenType::Module,
        TokenType::Identifier("basic".to_string()),
        TokenType::LeftBrace,
        TokenType::Let,
        TokenType::Identifier("x".to_string()),
        TokenType::Equals,
        TokenType::NumberLiteral(42.0),
        TokenType::Let,
        TokenType::Identifier("name".to_string()),
        TokenType::Equals,
        TokenType::StringLiteral("John".to_string()),
        TokenType::Let,
        TokenType::Identifier("active".to_string()),
        TokenType::Equals,
        TokenType::BooleanLiteral(true),
        TokenType::RightBrace,
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn test_types_file() {
    let source = fs::read_to_string("tests/test_files/types.j").unwrap();
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().unwrap();

    // Test type definitions
    let type_indices: Vec<usize> = tokens
        .iter()
        .enumerate()
        .filter(|(_, t)| t.token_type == TokenType::Type)
        .map(|(i, _)| i)
        .collect();

    assert_eq!(type_indices.len(), 2); // Should have two type definitions (Point and User)

    // Verify Point type structure
    let point_name_idx = type_indices[0] + 1;
    match &tokens[point_name_idx].token_type {
        TokenType::Identifier(name) => assert_eq!(name, "Point"),
        _ => panic!("Expected Point type name"),
    }

    // Verify User type structure
    let user_name_idx = type_indices[1] + 1;
    match &tokens[user_name_idx].token_type {
        TokenType::Identifier(name) => assert_eq!(name, "User"),
        _ => panic!("Expected User type name"),
    }
}

#[test]
fn test_constants_file() {
    let source = fs::read_to_string("tests/test_files/constants.j").unwrap();
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().unwrap();

    // Find all constant declarations
    let const_tokens: Vec<&Token> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Const)
        .collect();

    assert_eq!(const_tokens.len(), 5); // Should have 5 constants

    // Verify specific constant values
    let mut found_pi = false;
    let mut found_greeting = false;
    for i in 0..tokens.len() - 2 {
        if tokens[i].token_type == TokenType::Const {
            match &tokens[i + 1].token_type {
                TokenType::Identifier(name) if name == "PI" => match &tokens[i + 3].token_type {
                    TokenType::NumberLiteral(val) => {
                        assert!((val - 3.14159).abs() < 1e-5);
                        found_pi = true;
                    }
                    _ => panic!("Expected PI to be a number"),
                },
                TokenType::Identifier(name) if name == "GREETING" => {
                    match &tokens[i + 3].token_type {
                        TokenType::StringLiteral(val) => {
                            assert_eq!(val, "Hello, World!");
                            found_greeting = true;
                        }
                        _ => panic!("Expected GREETING to be a string"),
                    }
                }
                _ => {}
            }
        }
    }
    assert!(found_pi && found_greeting);
}

#[test]
fn test_complex_file() {
    let source = fs::read_to_string("tests/test_files/complex.j").unwrap();
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().unwrap();

    // Test nested type definitions
    let type_count = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Type)
        .count();
    assert_eq!(type_count, 2); // Vector3D and Matrix

    // Test brace matching
    let left_brace_count = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::LeftBrace)
        .count();
    let right_brace_count = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::RightBrace)
        .count();
    assert_eq!(left_brace_count, right_brace_count);
}

#[test]
fn test_error_file() {
    let source = fs::read_to_string("tests/test_files/errors.j").unwrap();
    let mut lexer = Lexer::new(&source);

    match lexer.tokenize() {
        Ok(_) => panic!("Expected error in error test file"),
        Err(e) => match e {
            LexerError::UnterminatedString { .. }
            | LexerError::InvalidNumber { .. }
            | LexerError::UnexpectedCharacter { .. } => (),
            _ => panic!("Unexpected error type"),
        },
    }
}

#[test]
fn test_comments_file() {
    let source = fs::read_to_string("tests/test_files/comments.j").unwrap();
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().unwrap();

    // Comments should be ignored, so check the actual tokens
    assert!(tokens.iter().all(|t| match &t.token_type {
        TokenType::Identifier(s) => !s.contains("//"),
        _ => true,
    }));
}

#[test]
fn test_whitespace_file() {
    let source = fs::read_to_string("tests/test_files/whitespace.j").unwrap();
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().unwrap();

    // Verify that extra whitespace doesn't affect token generation
    let compact_source =
        "module whitespace{type Space=>{x:Number,y:Number}let x=42 const PI=3.14159}";
    let mut compact_lexer = Lexer::new(compact_source);
    let compact_tokens = compact_lexer.tokenize().unwrap();

    assert_eq!(
        tokens.iter().map(|t| &t.token_type).collect::<Vec<_>>(),
        compact_tokens
            .iter()
            .map(|t| &t.token_type)
            .collect::<Vec<_>>()
    );
}

#[test]
fn test_all_tokens_file() {
    let source = fs::read_to_string("tests/test_files/all_tokens.j").unwrap();
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().unwrap();

    // Verify we have all token types
    let token_types: Vec<_> = tokens.iter().map(|t| &t.token_type).collect();

    // Check for presence of each token type
    assert!(token_types.iter().any(|t| matches!(t, TokenType::Module)));
    assert!(token_types.iter().any(|t| matches!(t, TokenType::Type)));
    assert!(token_types.iter().any(|t| matches!(t, TokenType::Let)));
    assert!(token_types.iter().any(|t| matches!(t, TokenType::Const)));
    assert!(token_types.iter().any(|t| matches!(t, TokenType::Number)));
    assert!(token_types.iter().any(|t| matches!(t, TokenType::String)));
    assert!(token_types.iter().any(|t| matches!(t, TokenType::Boolean)));
    assert!(
        token_types
            .iter()
            .any(|t| matches!(t, TokenType::NumberLiteral(_)))
    );
    assert!(
        token_types
            .iter()
            .any(|t| matches!(t, TokenType::StringLiteral(_)))
    );
    assert!(
        token_types
            .iter()
            .any(|t| matches!(t, TokenType::BooleanLiteral(_)))
    );
}

#[test]
fn test_unicode_file() {
    let source = fs::read_to_string("tests/test_files/unicode.j").unwrap();
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().unwrap();

    // Test basic string literals
    let string_literals: Vec<_> = tokens
        .iter()
        .filter_map(|t| match &t.token_type {
            TokenType::StringLiteral(s) => Some(s),
            _ => None,
        })
        .collect();

    assert!(string_literals.iter().any(|s| s.contains("Hello")));

    // Test identifiers
    let identifiers: Vec<_> = tokens
        .iter()
        .filter_map(|t| match &t.token_type {
            TokenType::Identifier(s) => Some(s.as_str()),
            _ => None,
        })
        .collect();

    assert!(identifiers.contains(&"pi"));
    assert!(identifiers.contains(&"theta"));
}
