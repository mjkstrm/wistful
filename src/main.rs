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
    /*
    match &input {
        Ok(_) => {
            match get_values(&input, &mut evaluator) {
                Ok(_) => println!("Evaluating succeeded."),
                Err(e) => println!("\x1b[0;31mParse error: {0}\x1b[0m", e)
                }
            }
        Err(error) => println!("error: {}", error),
    }*/
}



// Function to invoke Parser and evaluate expression
fn get_values(expr: &str, evaluator: &mut Evaluator) -> Result<(), ParseError> {
    // Vector of expressions to be evaluated
    let ast = Parser::new(&expr)?.parse()?;
    for expr in ast.iter() {
        evaluator.ast = Some(expr.clone());
        /*
        match evaluator.start_evaluating() {
            Ok(..) => Ok(()),
            Err(e) => return Err(e.into()),
        };*/
        evaluator.start_evaluating();
    }
    Ok(())
    /*
    // Assign evaluators ast now that we've parsed one.
    evaluator.ast = Some(ast.clone());
    match evaluator.start_evaluating() {
        Ok(..) => Ok(()),
        Err(e) => return Err(e.into()),
    }*/
}
