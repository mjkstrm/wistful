// Add modules
mod lexer_and_parser;
// Uses
use std::io;
// DEBUG use lexer_and_parser::tokenizer::Tokenizer as tokenizer;
use lexer_and_parser::ast;
use lexer_and_parser::parser::{ParseError, Parser};

fn main() {
    //parser::tokenizer::tokenize();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    //let _capture = evaluate(input);

    match evaluate(input) {
        Ok(val) => println!("> {:?}", val),
        Err(_) => println!("Couldnt evaluate"),
    }
}
// For Debugging purposes
fn print_tree(node: ast::Node, mut indent: String) -> () {
    //println!("{:?}", node);
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
            else {
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
    let expr = expr.split_whitespace().collect::<String>(); // remove whitespace chars
    let mut math_parser = Parser::new(&expr)?;
    let ast = math_parser.parse()?;

    //print_tree(ast.clone(), String::new());

    Ok(ast::eval(ast)?)
}
