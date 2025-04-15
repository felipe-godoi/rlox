use std::fmt::{self, Display};

use crate::{error_handler::RuntimeError, token_type::TokenType};

#[derive(PartialEq, Clone, Debug)]
pub enum LiteralType {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
    None,
}

impl LiteralType {
    pub fn as_number(&self, token: Token) -> Result<f64, RuntimeError> {
        if let LiteralType::Number(value) = self {
            return Ok(*value);
        } else {
            return Err(RuntimeError::new(token, "Operand must be a number"));
        };
    }
}

impl Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralType::String(s) => write!(f, "{}", s),
            LiteralType::Number(n) => write!(f, "{}", n),
            LiteralType::Bool(b) => write!(f, "{}", b),
            LiteralType::Nil => write!(f, "Nil"),
            LiteralType::None => write!(f, "None"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: LiteralType,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: LiteralType, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
}
