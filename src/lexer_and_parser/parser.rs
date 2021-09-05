use std::fmt;
// Internal uses
use super::tokenizer::Tokenizer;
use super::token::{Precedence, Token};
use super::ast::Node;

pub struct Parser<'a> {
    // input to be parsed
    tokenizer: Tokenizer<'a>,
    current_token: Token,
    pub nodes: Vec<Node>,
}

// Public methods
// impl<'a> declares the lifetime
// Parser<'a> uses the lifetime
impl<'a> Parser<'a> {
    // Create a new instance of Parser
    pub fn new(expr: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Tokenizer::new(expr);
        let cur_token = match lexer.next() {
            Some(token) => token,
            None => { 
                return Err(ParseError::InvalidOperator("Invalid character".into())) 
            },
        };
        Ok(Parser {
            tokenizer: lexer,
            current_token: cur_token,
            nodes: Vec::new(),
        })
    }

    // Method in the public interface for parsing the expression
    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let ast = self.generate_ast(Precedence::Default)?;
        Ok(ast)
    }
}

// Private methods
impl<'a> Parser<'a> {
    // Move on to the next token to be parsed.
    fn get_next_token(&mut self) -> Result<(), ParseError> {
        println!("CURRENT: {:?}", self.current_token);
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into()))
        };
        if next_token == Token::Whitespace {
            println!("Shouldnt be here..");
            self.get_next_token()?;
        }
        else {
            self.current_token = next_token;
        }
        Ok(())
    }

    fn generate_ast(&mut self, precedence: Precedence) -> Result<Node, ParseError> {
        let mut l_expr = self.get_primary_expression()?;
        
        // Start creating the tree recursively
        while precedence < self.current_token.get_precedence() {
            if self.current_token == Token::EOF {
                break;
            }
            
            let r_expr = self.parse_binary_expression(l_expr)?; 
            l_expr = r_expr;
        }
        Ok(l_expr)
    }

    // Parse primary expressions, Numbers, Negative values, Parentheses etc.
    fn get_primary_expression(&mut self) -> Result<Node, ParseError> {
        let token = self.current_token.clone();
        match token {
            // Retarded way to implement a negative integer
            Token::Subtract => {
                self.get_next_token()?;
                let expr = self.generate_ast(Precedence::NegativeValue)?;
                Ok(Node::NegativeNumberExpression(Box::new(expr)))
            }
            Token::Num(i) => {
                self.get_next_token()?;
                Ok(Node::NumberExpression(i))
            }
            Token::LeftParenthese => {
                self.get_next_token()?;
                let l_expr = self.generate_ast(Precedence::Default)?;
                self.check_paren(Token::RightParenthese)?;

                if self.current_token == Token::LeftParenthese {
                    let r_expr = self.generate_ast(Precedence::MultiplyAndDivide)?;
                    return Ok(Node::BinaryExpr { l_expr: Box::new(l_expr), operator: Token::Multiply, r_expr: Box::new(r_expr) })
                }

                Ok(l_expr)
            }
            Token::Literal(string) => {
                self.get_next_token()?;
                // Expecting an assignment after identifier
                self.check_token(Token::Assignment)?;
                let r_expr = self.generate_ast(Precedence::Default)?;
                let literal_expr = Node::LiteralExpression(string); 
                return Ok(Node::AssignmentExpression { identifier: Box::new(literal_expr), assignment_operator: Token::Assignment, expr: Box::new(r_expr) } )
            }
            _ => return Err(ParseError::InvalidOperator("Bad start".to_string()))
        }
    }

    // Closing parenthese is always expected, if not found return error
    fn check_paren(&mut self, right_paren: Token) -> Result<(), ParseError> {
        if right_paren == self.current_token {
            self.get_next_token()?;
            Ok(())
        } else {
            Err(ParseError::InvalidOperator(format!(
                "Expected {:?}, got {:?}",
                right_paren, self.current_token
            )))
        }
    }

    fn check_token(&mut self, expected_token: Token) -> Result<(), ParseError> {
        if expected_token == self.current_token {
            self.get_next_token()?;
            Ok(())
        }
        else {
            Err(ParseError::InvalidOperator(format!(
                "Expected {:?}, got {:?}",
                expected_token, self.current_token
            )))
        }
    }

    fn parse_binary_expression(&mut self, l_expr: Node) -> Result<Node, ParseError> {
        // Clone our current token
        let token = self.current_token.clone();
        match token {
            Token::Add => {
                let _capture = self.get_next_token();
                let r_expr = self.generate_ast(Precedence::AddAndSubtract)?;
                
                Ok(Node::BinaryExpr { l_expr: Box::new(l_expr), operator: Token::Add, r_expr: Box::new(r_expr) })
            }
            Token::Multiply => {
                let _capture = self.get_next_token();
                let r_expr = self.generate_ast(Precedence::MultiplyAndDivide)?;
                Ok(Node::BinaryExpr { l_expr: Box::new(l_expr), operator: Token::Multiply, r_expr: Box::new(r_expr) })
            },
            Token::Divide => {
                let _capture = self.get_next_token();
                let r_expr = self.generate_ast(Precedence::MultiplyAndDivide)?;
                Ok(Node::BinaryExpr { l_expr: Box::new(l_expr), operator: Token::Divide, r_expr: Box::new(r_expr) })
            }
            Token::Pow => {
                let _capture = self.get_next_token();
                let r_expr = self.generate_ast(Precedence::Power)?;
                Ok(Node::BinaryExpr { l_expr: Box::new(l_expr), operator: Token::Pow, r_expr: Box::new(r_expr) })
            }
            _ => return Err(ParseError::InvalidOperator("Bad token".to_string()))
        }
    }
}

// Handle error thrown from AST module
impl std::convert::From<std::boxed::Box<dyn std::error::Error>> for ParseError {
    fn from(_evalerr: std::boxed::Box<dyn std::error::Error>) -> Self {
        return ParseError::UnableToParse("Unable to parse".into());
    }
}
#[derive(Debug)]
pub enum ParseError {
    InvalidOperator(String),
    UnableToParse(String)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            self::ParseError::UnableToParse(e) => write!(f, "{}", e),
            self::ParseError::InvalidOperator(e) => write!(f, "{}", e),
        }
    }
}


