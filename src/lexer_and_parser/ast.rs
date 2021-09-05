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

// Enum holding the evaluate_numerics values
#[derive(Debug)]
pub enum EvalResult {
    Number(f64),
    //Assignment {
    //    identifier: Box<EvalResult>,
    //    value: Box<EvalResult>
    //}
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

pub fn evaluate(expr: Node) -> Result<EvalResult, Box<dyn error::Error>> {
    match expr {
        Node::NegativeNumberExpression(_) | Node::NumberExpression(_) => Ok(EvalResult::Number(evaluate_numerics(expr)?)),
        Node::BinaryExpr{ .. } => Ok(EvalResult::Number(evaluate_numerics(expr)?)),
        _ => Err("Couldnt evaluate".into())
    }

} 

// Evaluating arithmetics
fn evaluate_numerics(expr: Node) -> Result<f64, Box<dyn error::Error>> {
    use self::Node::*;
    match expr {
        NumberExpression(f) => Ok(f),
        NegativeNumberExpression(f) => Ok(-evaluate_numerics(*f)?),
        BinaryExpr { l_expr, operator, r_expr } => {
            match operator {
                Token::Add => Ok(evaluate_numerics(*l_expr)? + evaluate_numerics(*r_expr)?),
                Token::Multiply => Ok(evaluate_numerics(*l_expr)? * evaluate_numerics(*r_expr)?),
                Token::Divide => Ok(evaluate_numerics(*l_expr)? / evaluate_numerics(*r_expr)?),
                Token::Pow => Ok(evaluate_numerics(*l_expr)?.powf(evaluate_numerics(*r_expr)?)),
                // Fix this, bad implementation
                _ => Err("Couldnt evaluate".into())
            }
        }
        _ => Err("Not implemented.".into())
    }
}
