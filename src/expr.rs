use crate::token::{LiteralType, Token};

#[derive(Debug)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

#[derive(Debug)]
pub struct Literal {
    pub value: LiteralType,
}

#[derive(Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct Comma {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Comma(Comma),
}

pub trait Visitor<T> {
    fn visit(expr: Expr) -> T;
}
