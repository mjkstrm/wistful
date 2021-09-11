// Add internal modules & std
mod lexer_and_parser;
use std::io;
use lexer_and_parser::ast;
use lexer_and_parser::parser::{ParseError, Parser};

fn main() {
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

// For Debugging purposes, allow(dead_code) removes warnings from unused code. NOT UP TO DATE..
#[allow(dead_code)]
fn print_tree(node: ast::Node, mut indent: String) -> () {
    if indent.is_empty() {
        indent = "├──".to_string();
    }
    match node {
        ast::Node::BinaryExpr {
            ref l_expr,
            ref operator,
            ref r_expr,
        } => {
            print_tree(*l_expr.clone(), indent.clone());
            if l_expr.is_binary_expr() {
            } 
            else
            {
                println!("{0}{1:?}", indent, l_expr);
            }

            println!("{0}{1:?}", indent, operator);
            println!("{0}{1:?}", indent, r_expr);
            indent = "   ├─".to_string();
            print_tree(*r_expr.clone(), indent);
        }
        _ => return,
    }
}

// Function to invoke Parser and evaluate expression
fn get_values(expr: &str) -> Result<(), ParseError> {
    let mut parser = Parser::new(&expr)?;
    let ast = parser.parse()?;
    let test = ast::evaluate(ast.clone())?;
    println!("{:?}", test);
    //
    match ast::evaluate(ast) {
        Ok(..) => Ok(()),
        Err(e) => return Err(e.into()),
    }
}
