// Add modules
mod lexer_and_parser;
// Uses
use std::io;
// DEBUG use lexer_and_parser::tokenizer::Tokenizer as tokenizer;
use lexer_and_parser::ast::Node;
use lexer_and_parser::parser::{ParseError, Parser};

fn main() {
    //parser::tokenizer::tokenize();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let _capture = evaluate(input);
}

fn print_tree(node: Node, mut indent: String) -> () {
    if indent.is_empty() {
        indent = " ".to_string();
    }
    match node {
        // Return if this is the last node
        Node::Number(n) => {
            println!("{0} {1}", indent, n);
            return;
        },
        Node::Add(ref l_expr, ref r_expr) => {
            print_tree(*l_expr.clone(), indent.clone());
            indent = " ".to_string();
            println!("{0} +", indent);
            print_tree(*r_expr.clone(), indent.clone());
        },
        Node::Multiply(ref l_expr, ref r_expr) => {
            indent = "    ".to_string();
            print_tree(*l_expr.clone(), indent.clone());
            println!("{0} *", indent);
            print_tree(*r_expr.clone(), indent.clone());
        }
        Node::Divide(ref l_expr, ref r_expr) => {
            indent = "    ".to_string();
            print_tree(*l_expr.clone(), indent.clone());
            println!("{0} /", indent);
            print_tree(*r_expr.clone(), indent.clone());
        }
        _ => return
    }
}

// Function to invoke Parser and evaluate expression
fn evaluate(expr: String) -> Result<f64, ParseError> {
    // Remove all whitespace
    let expr = expr.split_whitespace().collect::<String>();
    let mut math_parser = Parser::new(&expr)?;
    let ast = math_parser.parse()?;

    //EXPERIMENTAL
    let mut i = 0;
    let mut node = ast;
    print_tree(node, String::new());

    let nr: f64 = 12.0;
    Ok(nr)
}
