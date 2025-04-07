use crate::{
    error_handler::ErrorHandler,
    expr::{Binary, Comma, Expr, Grouping, Literal, Ternary, Unary},
    token::{LiteralType, Token},
    token_type::TokenType,
};

pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: usize,
    error_handler: &'a mut ErrorHandler,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, error_handler: &'a mut ErrorHandler) -> Self {
        Self {
            current: 0,
            tokens,
            error_handler,
        }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        let result = self.expression();

        match result {
            Ok(result) => Some(result),
            Err(_) => None,
        }
    }

    fn comma(&mut self) -> Result<Expr, String> {
        let mut expr = self.expression()?;

        while self.match_token(vec![TokenType::Comma]) {
            let right = self.expression()?;
            expr = Expr::Comma(Comma {
                left: Box::new(expr),
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr, String> {
        let expr = self.ternary()?;

        return Ok(expr);
    }

    fn ternary(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;

        if self.match_token(vec![TokenType::Question]) {
            let condition = expr;
            let then_branch = self.equality()?;
            self.consume(&TokenType::Colon, "Expect ':' after then branch.")?;
            let else_branch = self.ternary()?;
            expr = Expr::Ternary(Ternary {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch: Box::new(else_branch),
            })
        };

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_token(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::from(right),
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(Unary {
                operator,
                right: Box::new(right),
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(vec![TokenType::False]) {
            return Ok(Expr::Literal(Literal {
                value: LiteralType::Bool(false),
            }));
        };

        if self.match_token(vec![TokenType::True]) {
            return Ok(Expr::Literal(Literal {
                value: LiteralType::Bool(true),
            }));
        }

        if self.match_token(vec![TokenType::Nil]) {
            return Ok(Expr::Literal(Literal {
                value: LiteralType::Nil,
            }));
        }

        if self.match_token(vec![TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(Literal {
                value: self.previous().literal,
            }));
        }

        if self.match_token(vec![TokenType::LeftParen]) {
            let comma = self.comma()?;
            self.consume(&TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Grouping {
                expression: Box::new(comma),
            }));
        }

        Err(self.error(&self.peek(), "Expect expression."))
    }

    fn match_token(&mut self, types: Vec<TokenType>) -> bool {
        let is_matched = types.iter().any(|token_type| self.check(token_type));

        if is_matched {
            self.advance();
            return true;
        }
        false
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<Token, String> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(self.error(&self.peek(), message))
    }

    fn error(&mut self, token: &Token, message: &str) -> String {
        self.error_handler.error_with_token(token, message);
        message.to_string()
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return (),
                _ => self.advance(),
            };
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        &self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens.iter().nth(self.current).unwrap().clone()
    }

    fn previous(&mut self) -> Token {
        self.tokens.iter().nth(self.current - 1).unwrap().clone()
    }
}
