use crate::{
    expr::{Expr, Grouping, Literal, Unary, Visitor},
    token::LiteralType,
    token_type::TokenType,
};

struct Interpreter {}

impl Interpreter {
    fn evaluate(expr: Expr) -> LiteralType {
        Interpreter::visit(expr)
    }

    fn evaluate_unary(unary: Unary) -> LiteralType {
        let Unary { operator, right } = unary;

        let evaluated_right = Interpreter::evaluate(*right);

        match operator.token_type {
            TokenType::Minus => {
                if let LiteralType::Number(value) = evaluated_right {
                    return LiteralType::Number(-value);
                }

                LiteralType::None
            }
            TokenType::Bang => todo!(),
            _ => unreachable!(),
        }
    }
}

impl Visitor<LiteralType> for Interpreter {
    fn visit(expr: Expr) -> LiteralType {
        match expr {
            Expr::Literal(Literal { value }) => value,
            Expr::Grouping(Grouping { expression }) => Interpreter::evaluate(*expression),
            Expr::Unary(unary) => Interpreter::evaluate_unary(unary),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_literal() {
        let expr = Expr::Literal(Literal {
            value: LiteralType::String(String::from("Teste")),
        });

        let result = Interpreter::visit(expr);

        assert_eq!(result, LiteralType::String(String::from("Teste")));
    }

    #[test]
    fn test_grouping() {
        let expr = Expr::Grouping(Grouping {
            expression: Box::new(Expr::Literal(Literal {
                value: LiteralType::Number(123.into()),
            })),
        });

        let result = Interpreter::visit(expr);

        assert_eq!(result, LiteralType::Number(123.into()));
    }
}
