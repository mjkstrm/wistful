// Add internal modules & std
mod lexer_and_parser;
use std::io;
use lexer_and_parser::parser::{ParseError, Parser};
use lexer_and_parser::evaluator::{Evaluator};

fn main() {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    match get_values(&input) {
                        Ok(_) => println!("Evaluating succeeded."),
                        Err(e) => println!("\x1b[0;31mParse error: {0}\x1b[0m", e)
                        }
                    }
                Err(error) => println!("error: {}", error),
        }
    }
}

// Function to invoke Parser and evaluate expression
fn get_values(expr: &str) -> Result<(), ParseError> {
    let ast = Parser::new(&expr)?.parse()?;
    println!("{:?}", ast);
    let mut evaluator = Evaluator::new(ast);
    match evaluator.start_evaluating() {
        Ok(..) => {
            for var in evaluator.variable_storage.iter() {
                println!("{0:?} equals {1:?}", var.name, var.value);
            }
            Ok(())
        },
        Err(e) => return Err(e.into()),
    }
}
