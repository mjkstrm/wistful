use std::fs;

use lexer_and_parser::evaluator::Evaluator;
use lexer_and_parser::parser::{ParseError, Parser};

// Add internal modules & std
mod lexer_and_parser;

fn main() {
    // Initialize evaluator.
    let mut evaluator: Evaluator = Evaluator::new(None);
    let input =
        fs::read_to_string("test-source").expect("\x1b[0;31mTest source was not found.\x1b[0m");
    println!("{0:?}", input);
    match get_values(
        &input,
        &mut evaluator,
    ) {
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
        evaluator.ast = Some(expression);
        evaluator.start_evaluating()?;
    }
    Ok(())
}
