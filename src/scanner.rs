use crate::{
    error_handler::ErrorHandler,
    token::{LiteralType, Token},
    token_type::TokenType,
};

pub struct Scanner<'a> {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    error_handler: &'a mut ErrorHandler,
}

fn keywords(key: &str) -> TokenType {
    match key {
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "for" => TokenType::For,
        "fun" => TokenType::Fun,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "var" => TokenType::Var,
        "while" => TokenType::While,
        _ => TokenType::Identifier,
    }
}

impl<'a> Scanner<'a> {
    pub fn new(source: String, error_handler: &'a mut ErrorHandler) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            error_handler,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            String::from(""),
            LiteralType::None,
            self.line,
        ));

        &self.tokens
    }

    fn scan_token(&mut self) {
        let char = self.advance().unwrap();

        match char {
            '(' => self.add_token(TokenType::LeftParen, LiteralType::None),
            ')' => self.add_token(TokenType::RightParen, LiteralType::None),
            '{' => self.add_token(TokenType::LeftBrace, LiteralType::None),
            '}' => self.add_token(TokenType::RightBrace, LiteralType::None),
            ',' => self.add_token(TokenType::Comma, LiteralType::None),
            '.' => self.add_token(TokenType::Dot, LiteralType::None),
            '-' => self.add_token(TokenType::Minus, LiteralType::None),
            '+' => self.add_token(TokenType::Plus, LiteralType::None),
            ';' => self.add_token(TokenType::Semicolon, LiteralType::None),
            '*' => self.add_token(TokenType::Star, LiteralType::None),
            '!' => {
                let next_is_equal = self.match_char('=');
                self.add_token(
                    if next_is_equal {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    },
                    LiteralType::None,
                )
            }
            '=' => {
                let next_is_equal = self.match_char('=');
                self.add_token(
                    if next_is_equal {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    },
                    LiteralType::None,
                )
            }
            '<' => {
                let next_is_equal = self.match_char('=');
                self.add_token(
                    if next_is_equal {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    },
                    LiteralType::None,
                )
            }
            '>' => {
                let next_is_equal = self.match_char('=');
                self.add_token(
                    if next_is_equal {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    },
                    LiteralType::None,
                )
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    println!("Multiline comment");
                    self.ignore_multiline_comment();
                } else {
                    self.add_token(TokenType::Slash, LiteralType::None);
                }
            }
            '"' => self.add_string(),
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            c => {
                if c.is_ascii_digit() {
                    self.add_number();
                } else if self.is_alpha(c) {
                    self.add_identifier();
                } else {
                    self.error_handler
                        .error(self.line, &format!("Unexpected character. {}", c));
                }
            }
        }
    }

    fn ignore_multiline_comment(&mut self) {
        let mut nested_comments = 0;

        while (self.peek() != '*' || self.peek_next() != '/') && !self.is_at_end() {
            if self.peek() == '/' && self.peek_next() == '*' {
                nested_comments += 1;
            }

            if self.peek() == '\n' {
                self.line += 1;
            };

            self.advance();

            if self.peek() == '*' && self.peek_next() == '/' && nested_comments > 0 {
                self.advance();
                self.advance();
                nested_comments -= 1;
            };
        }

        if self.is_at_end() {
            self.error_handler
                .error(self.line, "Unterminated comment block");
            return;
        }

        self.advance();
        self.advance();
    }

    fn add_identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let token_type = keywords(&self.source[self.start..self.current]);

        self.add_token(token_type, LiteralType::None);
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || c.is_ascii_digit()
    }

    fn add_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value = &self.source[self.start..self.current];

        self.add_token(
            TokenType::Number,
            LiteralType::Number(str::parse::<f64>(value).unwrap()),
        )
    }

    fn add_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            };
            self.advance();
        }

        if self.is_at_end() {
            self.error_handler.error(self.line, "Unterminated string");
            return;
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1]
            .to_string()
            .clone();

        self.add_token(TokenType::String, LiteralType::String(value))
    }

    fn add_token(&mut self, token_type: TokenType, literal: LiteralType) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line))
    }

    fn advance(&mut self) -> Option<char> {
        let char = self.source.chars().nth(self.current);
        self.current += 1;

        char
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }
}
