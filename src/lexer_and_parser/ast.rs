// Internal modules
use super::token::{Token, Keyword};

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
        r_expr: Box<Node>
    },
    AssignmentExpression {
        identifier: Box<Node>,
        assignment_operator: Token,
        expr: Box<Node>
    },
   
}


// Check wheter current node is an binary node NOT USED ATM
impl Node {
    pub fn is_binary_expr(&self) -> bool {
        match self.clone() {
            Node::BinaryExpr { l_expr: _, operator: _, r_expr: _ } => true,
            _ => false
        }
    }
}

