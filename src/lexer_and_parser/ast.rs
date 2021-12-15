// Internal modules
use super::token::{Keyword, Token};

// Nodes for the syntax tree
#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    NumberExpression(f64),
    NegativeNumberExpression(Box<Node>),
    LiteralExpression(String, Keyword),
    IdentifierExpression(String),
    BinaryExpr {
        l_expr: Box<Node>,
        operator: Token,
        r_expr: Box<Node>,
    },
    AssignmentExpression {
        identifier: Box<Node>,
        assignment_operator: Token,
        expr: Box<Node>,
    },
    ConditionExpression {
        l_expr: Box<Node>,
        operator: Token,
        r_expr: Box<Node>,
    },
    IfExpression {
        condition: Box<Option<Node>>,
        // Contains one block of statements
        then_branch: Box<Vec<Node>>,
        // Contains n amount of else cases including each cases block of statements.
        else_branch: Box<Option<Node>>,
    },
    ElseExpression {
        condition: Box<Option<Node>>,
        then_branch: Box<Vec<Node>>,
        else_branch: Box<Option<Node>>,
    },
    WhileExpression{
        condition: Box<Option<Node>>,
        then_branch: Box<Vec<Node>>
    },
    EOF(String),
}

impl Node {
    pub fn print_stuff(&self, expr: Node, indent: i16) -> String {
        let mut i = 0;
        let mut indent_str = String::new();
        while i < indent {
            indent_str += " ";
            i += 1;
        }
        indent_str += "-";
        match expr {
            Node::NumberExpression(f) => {
                println!("\x1b[0;34m{0}Number: {1} \x1b[0m", indent_str, f);
                return format!("{0}Number: {1}", indent_str, f);
            }
            Node::NegativeNumberExpression(f) => {
                println!("\x1b[0;34m{0}NegativeNumber: {1:?} \x1b[0m", indent_str, f);
            }
            Node::LiteralExpression(id, _) => {
                println!("\x1b[0;34m{0}Literal: {1} \x1b[0m", indent_str, id);
                return format!("{0}Literal: {1}", indent_str, id);
            }
            Node::IdentifierExpression(id) => {
                println!("\x1b[0;34m{0}Identifier: {1} \x1b[0m", indent_str, id);
                return format!("{0}Identifier: {1}", indent_str, id);
            }
            Node::BinaryExpr { l_expr, operator, r_expr } => {
                println!("\x1b[0;32m{0}BinaryExpression: \x1b[0m", indent_str);
                self.print_stuff(*l_expr, indent + 1);
                println!(" \x1b[0;34m{0}{1:?} \x1b[0m", indent_str, operator);
                self.print_stuff(*r_expr, indent + 1);
            }
            Node::AssignmentExpression { identifier, assignment_operator, expr } => {
                println!("\x1b[0;32m{0}AssignmentExpression: \x1b[0m", indent_str);
                self.print_stuff(*identifier, indent + 1);
                println!(" \x1b[0;34m{0}{1:?} \x1b[0m", indent_str, assignment_operator);
                self.print_stuff(*expr, indent + 1);
            }
            Node::ConditionExpression { l_expr, operator, r_expr } => {
                self.print_stuff(*l_expr, indent + 1);
                println!(" \x1b[0;32m{0}{1:?} \x1b[0m", indent_str, operator);
                self.print_stuff(*r_expr, indent + 1);
            }
            Node::IfExpression { condition, then_branch, else_branch } => {
                let node = condition.unwrap();
                println!("\x1b[0;32m{0}IfExpression: \x1b[0m", indent_str);
                self.print_stuff(node, indent);
                println!("  \x1b[0;32m{0}THEN \x1b[0m", indent_str);
                for node in then_branch.into_iter() {
                    self.print_stuff(node, indent + 3);
                }
                if else_branch.is_some() {
                    let else_node = else_branch.unwrap();
                    self.print_stuff(else_node, indent + 1);
                }
            }
            Node::ElseExpression { condition, then_branch, else_branch } => {
                if condition.is_some()
                {
                    let node = condition.unwrap();
                    println!("\x1b[0;32m{0}ElseIfExpression: \x1b[0m", indent_str);
                    self.print_stuff(node, indent + 1);
                    println!("   \x1b[0;32m{0}THEN: \x1b[0m", indent_str);
                } else if condition.is_none() {
                    println!("\x1b[0;32m{0}ElseExpression: \x1b[0m", indent_str);
                }
                for node in then_branch.into_iter() {
                    self.print_stuff(node, indent + 4);
                }
                if else_branch.is_some() {
                    let else_node = else_branch.unwrap();
                    self.print_stuff(else_node, indent + 1);
                }
            }
            Node::EOF(_) => {
                return format!("Hm");
            }
            _ => { return format!("Not handled"); }
        };
        return "".to_string();
    }
}
