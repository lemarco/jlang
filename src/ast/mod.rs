#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    String,
    Boolean,
    Custom(String), // For user-defined types
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    NumberLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    Identifier(String),
    Object { fields: Vec<(String, Expression)> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeField {
    pub name: String,
    pub field_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDefinition {
    pub name: String,
    pub fields: Vec<TypeField>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let {
        name: String,
        value: Box<Expression>,
    },
    Const {
        name: String,
        value: Box<Expression>,
    },
    TypeDef(TypeDefinition),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub name: String,
    pub statements: Vec<Statement>,
}

// Root node of our AST
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub modules: Vec<Module>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            modules: Vec::new(),
        }
    }
}
