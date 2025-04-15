use std::{
    fs,
    io::{self, BufRead, BufReader, Write},
    process,
};

use crate::{
    ast_printer::AstPrinter, error_handler::ErrorHandler, interpreter::Interpreter, parser::Parser,
    scanner::Scanner,
};

pub struct Program {
    error_handler: ErrorHandler,
}

impl Program {
    pub fn new() -> Self {
        let error_handler = ErrorHandler::new();

        Self { error_handler }
    }

    pub fn init(&mut self, file: Option<String>) {
        if let Some(script) = file {
            self.run_file(&script);
        } else {
            self.run_prompt();
        }
    }

    fn run_file(&mut self, path: &str) {
        let content =
            fs::read_to_string(path).expect("Erro ao ler o arquivo! Informe um caminho válido.");

        if self.error_handler.had_error {
            process::exit(65);
        }

        if self.error_handler.had_runtime_error {
            process::exit(70);
        }

        self.run(&content);
    }

    fn run_prompt(&mut self) {
        let stdin = io::stdin();
        let handle = stdin.lock();

        let reader = BufReader::new(handle);

        print!("> ");
        io::stdout().flush().unwrap();

        for line in reader.lines() {
            match line {
                Ok(text) => {
                    self.run(&text);
                }
                Err(e) => eprintln!("Error reading line: {}", e),
            }

            print!("> ");
            io::stdout().flush().unwrap();
            self.error_handler.had_error = false;
        }
    }

    fn run(&mut self, source: &str) {
        let mut scanner = Scanner::new(source.to_string(), &mut self.error_handler);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens.to_owned(), &mut self.error_handler);
        let expression = parser.parse();

        if self.error_handler.had_error {
            return;
        }

        let mut interpreter = Interpreter::new(&mut self.error_handler);

        if let Some(expr) = expression {
            // AstPrinter::print(expr);
            interpreter.interpret(expr);
        };
    }
}
