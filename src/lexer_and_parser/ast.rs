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
    EOF(String),
}
