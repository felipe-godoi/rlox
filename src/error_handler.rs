use crate::{token::Token, token_type::TokenType};

pub struct ErrorHandler {
    pub had_error: bool,
}

impl ErrorHandler {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    pub fn report(&mut self, line: usize, _where: &str, message: &str) {
        eprintln!("[line: {}] Error {}: {}", line, _where, message);
        self.had_error = true;
    }

    pub fn error_with_token(&mut self, token: &Token, message: &str) {
        if token.token_type == TokenType::EOF {
            self.report(token.line, "at end", message);
        } else {
            self.report(token.line, &format!(" at'{}'", token.lexeme), message)
        }
    }
}
