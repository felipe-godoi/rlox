use crate::{
    error_handler::{ErrorHandler, RuntimeError},
    expr::{Binary, Expr, Grouping, Literal, Unary, Visitor},
    token::LiteralType,
    token_type::TokenType,
};

pub struct Interpreter<'a> {
    error_handler: &'a mut ErrorHandler,
}

impl<'a> Interpreter<'a> {
    pub fn new(error_handler: &'a mut ErrorHandler) -> Self {
        Self { error_handler }
    }

    pub fn interpret(&mut self, expression: Expr) {
        let value = Self::evaluate(expression);

        match value {
            Ok(result) => println!("{:?}", result),
            Err(err) => self.error_handler.runtime_error(err),
        }
    }

    fn evaluate(expr: Expr) -> Result<LiteralType, RuntimeError> {
        Interpreter::visit(expr)
    }

    fn evaluate_unary(unary: Unary) -> Result<LiteralType, RuntimeError> {
        let Unary { operator, right } = unary;

        let evaluated_right = Interpreter::evaluate(*right)?;

        match operator.token_type {
            TokenType::Minus => {
                return Ok(LiteralType::Number(-evaluated_right.as_number(operator)?));
            }
            TokenType::Bang => Ok(LiteralType::Bool(Interpreter::is_truthy(evaluated_right))),
            _ => unreachable!(),
        }
    }

    fn evaluate_binary(binary: Binary) -> Result<LiteralType, RuntimeError> {
        let Binary {
            left,
            operator,
            right,
        } = binary;

        let evaluated_left = Interpreter::evaluate(*left)?;
        let evaluated_right = Interpreter::evaluate(*right)?;

        match operator.token_type {
            TokenType::Minus => Ok(LiteralType::Number(
                evaluated_left.as_number(operator.clone())?
                    - evaluated_right.as_number(operator)?,
            )),
            TokenType::Slash => Ok(LiteralType::Number(
                evaluated_left.as_number(operator.clone())?
                    / evaluated_right.as_number(operator)?,
            )),
            TokenType::Star => Ok(LiteralType::Number(
                evaluated_left.as_number(operator.clone())?
                    * evaluated_right.as_number(operator)?,
            )),
            TokenType::Plus => match (evaluated_left, evaluated_right) {
                (LiteralType::Number(left_value), LiteralType::Number(right_value)) => {
                    Ok(LiteralType::Number(left_value + right_value))
                }
                (LiteralType::String(left_value), LiteralType::String(right_value)) => {
                    Ok(LiteralType::String(left_value + &right_value))
                }
                _ => Err(RuntimeError::new(
                    operator,
                    "Operands must be two numbers or two strings.",
                )),
            },
            TokenType::Greater => Ok(LiteralType::Bool(
                evaluated_left.as_number(operator.clone())?
                    > evaluated_right.as_number(operator)?,
            )),
            TokenType::GreaterEqual => Ok(LiteralType::Bool(
                evaluated_left.as_number(operator.clone())?
                    >= evaluated_right.as_number(operator)?,
            )),
            TokenType::Less => Ok(LiteralType::Bool(
                evaluated_left.as_number(operator.clone())?
                    < evaluated_right.as_number(operator)?,
            )),
            TokenType::LessEqual => Ok(LiteralType::Bool(
                evaluated_left.as_number(operator.clone())?
                    <= evaluated_right.as_number(operator)?,
            )),
            TokenType::BangEqual => Ok(LiteralType::Bool(evaluated_left != evaluated_right)),
            TokenType::EqualEqual => Ok(LiteralType::Bool(evaluated_left == evaluated_right)),
            _ => unreachable!(),
        }
    }

    fn is_truthy(literal: LiteralType) -> bool {
        match literal {
            LiteralType::Nil => return false,
            LiteralType::Bool(value) => return value,
            _ => true,
        }
    }
}

impl<'a> Visitor<Result<LiteralType, RuntimeError>> for Interpreter<'a> {
    fn visit(expr: Expr) -> Result<LiteralType, RuntimeError> {
        match expr {
            Expr::Literal(Literal { value }) => Ok(value),
            Expr::Grouping(Grouping { expression }) => Interpreter::evaluate(*expression),
            Expr::Unary(unary) => Interpreter::evaluate_unary(unary),
            Expr::Binary(binary) => Interpreter::evaluate_binary(binary),
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

        assert_eq!(result.unwrap(), LiteralType::String(String::from("Teste")));
    }

    #[test]
    fn test_grouping() {
        let expr = Expr::Grouping(Grouping {
            expression: Box::new(Expr::Literal(Literal {
                value: LiteralType::Number(123.into()),
            })),
        });

        let result = Interpreter::visit(expr);

        assert_eq!(result.unwrap(), LiteralType::Number(123.into()));
    }
}
