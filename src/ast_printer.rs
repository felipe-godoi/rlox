use crate::expr::{Binary, Expr, Grouping, Literal, Unary, Visitor};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(expr: Expr) -> String {
        let output = AstPrinter::visit(expr);
        println!("{}", output);

        output
    }

    fn parenthesize(name: &str, exprs: Vec<Box<Expr>>) -> String {
        let mut result = String::new();

        result.push_str("(");
        result.push_str(name);

        for expr in exprs {
            result.push_str(" ");
            result.push_str(&AstPrinter::visit(*expr));
        }

        result.push_str(")");

        result
    }
}

impl Visitor<String> for AstPrinter {
    fn visit(expr: Expr) -> String {
        match expr {
            Expr::Binary(Binary {
                left,
                operator,
                right,
            }) => AstPrinter::parenthesize(&operator.lexeme, vec![left, right]),
            Expr::Grouping(Grouping { expression }) => {
                AstPrinter::parenthesize("group", vec![expression])
            }
            Expr::Literal(Literal { value }) => format!("{}", value),
            Expr::Unary(Unary { operator, right }) => {
                AstPrinter::parenthesize(&operator.lexeme, vec![right])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        token::{LiteralType, Token},
        token_type::TokenType,
    };

    use super::*;

    #[test]
    fn test_ast_printer() {
        let expr = Expr::Binary(Binary {
            left: Box::new(Expr::Unary(Unary {
                operator: Token::new(TokenType::Minus, "-".to_string(), LiteralType::None, 1),
                right: Box::new(Expr::Literal(Literal {
                    value: LiteralType::Number(123.0),
                })),
            })),
            operator: Token::new(TokenType::Star, "*".to_string(), LiteralType::None, 1),
            right: Box::new(Expr::Grouping(Grouping {
                expression: Box::new(Expr::Literal(Literal {
                    value: LiteralType::Number(45.67),
                })),
            })),
        });

        let result = AstPrinter::print(expr);

        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}
