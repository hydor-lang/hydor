use std::time::Instant;

use crate::{lexer::Lexer, parser::parser::Parser};

pub fn run_file(source: String) {
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);

    match parser.parse_program() {
        Ok(program) => {
            println!("{:#?}", program)
        }
        Err(ec) => {
            ec.report_all(&source);
        }
    }
}
