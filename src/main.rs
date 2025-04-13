use clap::Parser;
use program::Program;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    generate: Option<String>,

    script: Option<String>,
}

mod ast_printer;
mod error_handler;
mod expr;
mod interpreter;
mod parser;
mod program;
mod scanner;
mod token;
mod token_type;
mod tool;

fn main() {
    let args = Args::parse();

    if args.generate.is_some() {
        tool::generate_ast::generate_ast(&args.generate.unwrap()).expect("Error generating AST");
        return;
    }

    let mut interpreter = Program::new();

    interpreter.init(args.script);
}
