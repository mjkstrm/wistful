// Libraries
use std::error;

// Internal modules
use super::token::Token;
// Nodes for the syntax tree

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    NumberExpression(f64),
    NegativeNumberExpression(Box<Node>),
    LiteralExpression(String),
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

// Check wheter current node is an binary node
impl Node {
    pub fn is_binary_expr(&self) -> bool {
        match self.clone() {
            Node::BinaryExpr { l_expr: _, operator: _, r_expr: _ } => true,
            _ => false
        }
    }
}

// Evaluating arithmetics
pub fn eval(expr: Node) -> Result<f64, Box<dyn error::Error>> {
    use self::Node::*;
    match expr {
        NumberExpression(f) => Ok(f),
        NegativeNumberExpression(f) => Ok(-eval(*f)?),
        BinaryExpr { l_expr, operator, r_expr } => {
            match operator {
                Token::Add => Ok(eval(*l_expr)? + eval(*r_expr)?),
                Token::Multiply => Ok(eval(*l_expr)? * eval(*r_expr)?),
                Token::Divide => Ok(eval(*l_expr)? / eval(*r_expr)?),
                Token::Pow => Ok(eval(*l_expr)?.powf(eval(*r_expr)?)),
                // Fix this, bad implementation
                _ => Err("Couldnt evaluate".into())
            }
        },
        _ => None
    }
}
