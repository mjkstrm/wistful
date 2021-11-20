// Add internal modules & std
mod lexer_and_parser;
use lexer_and_parser::evaluator::Evaluator;
use lexer_and_parser::parser::{ParseError, Parser};
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    // Initialize evaluator.
    let mut evaluator = Evaluator::new(None);
    let mut input =
        fs::read_to_string("/Users/mjkstrm/wistful-test/test.txt").expect("Could not read file.");
    println!("{0:?}", input);
    match get_values(&input, &mut evaluator) {
        Ok(_) => println!("Evaluating succeeded."),
        Err(e) => println!("\x1b[0;31mParse error: {0}\x1b[0m", e),
    }
}

// Function to invoke Parser and evaluate expression
fn get_values(expr: &str, evaluator: &mut Evaluator) -> Result<(), ParseError> {
    // Vector of expressions to be evaluated
    let mut expressions = Parser::new(&expr)?.parse()?;
    // Capture each removed node to a variable and feed it to evaluator to avoid
    // borrowing/cloning of values.
    while expressions.len() > 0 {
        let expression = expressions.remove(0);
        println!("{0:?}", expression);
        evaluator.ast = Some(expression);
        evaluator.start_evaluating();
    }
    Ok(())
}
