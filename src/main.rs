// Add internal modules & std
mod lexer_and_parser;
use std::io;
use lexer_and_parser::ast;
use lexer_and_parser::parser::{ParseError, Parser};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match evaluate(input) {
        Ok(val) => println!("> {:?}", val),
        Err(_) => println!("Could not evaluate"),
    }
}
// For Debugging purposes
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
fn evaluate(expr: String) -> Result<f64, ParseError> {
    // Remove whitespace chars
    let expr = expr.split_whitespace().collect::<String>();
    let mut math_parser = Parser::new(&expr)?;
    let ast = math_parser.parse()?;
    // Print out the syntax tree for debuggin purposes
    //print_tree(ast.clone(), String::new());

    Ok(ast::eval(ast)?)
}
