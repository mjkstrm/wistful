// Add internal modules & std
mod lexer_and_parser;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use lexer_and_parser::parser::{ParseError, Parser};
use lexer_and_parser::evaluator::{Evaluator};
fn main() {
    // Initialize evaluator.
    let mut evaluator = Evaluator::new(None);
    let mut input = fs::read_to_string("/Users/mjkstrm/wistful-test/test.txt").expect("Could not read file.");
    println!("{0:?}", input);
    match get_values(&input, &mut evaluator) {
        Ok(_) => println!("Evaluating succeeded."),
        Err(e) => println!("\x1b[0;31mParse error: {0}\x1b[0m", e)
    }
}



// Function to invoke Parser and evaluate expression
fn get_values(expr: &str, evaluator: &mut Evaluator) -> Result<(), ParseError> {
    // Vector of expressions to be evaluated
    let mut expressions = Parser::new(&expr)?.parse()?;
    // Evaluate each given "syntax tree".
    // Using vectors pop function to move the value out from vector, avoiding unnecessary
    // borrowing/cloning of values.
    while expressions.len() > 0 {
        evaluator.ast = Some(expressions.pop().unwrap());
        evaluator.start_evaluating();
    } 
    Ok(())
}
