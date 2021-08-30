

// Uses
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
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        Ok(Parser {
            tokenizer: lexer,
            current_token: cur_token,
            nodes: Vec::new(),
        })
    }

    // Method in the public interface for parsing the expression
    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let ast = self.generate_ast()?;

        

        Ok(ast)
    }
}

// Private methods
impl<'a> Parser<'a> {
    // Move on to the next token to be parsed.
    fn get_next_token(&mut self) -> Result<(), ParseError> {
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".to_string()))
        };
        self.current_token = next_token;
        Ok(())
    }

    fn generate_ast(&mut self) -> Result<Node, ParseError> {
        /* 
        while self.current_token != Token::EOF {
            self.generate_ast();
        }*/
        let mut l_expr = self.parse_primary_expr()?;
        // Start creating the tree recursively
        while self.current_token != Token::EOF {
            let r_expr = self.parse_expr(l_expr)?;
            self.nodes.push(r_expr.clone());
            l_expr = r_expr;   
        }
        
        Ok(l_expr)
    }

    fn parse_primary_expr(&mut self) -> Result<Node, ParseError> {
        let token = self.current_token.clone();
        match token {
            Token::Num(i) => {
                let _capture = self.get_next_token();
                Ok(Node::Number(i))
            }
            _ => return Err(ParseError::InvalidOperator("Bad start".to_string()))
        }
    }

    fn parse_expr(&mut self, l_expr: Node) -> Result<Node, ParseError> {
        // Clone our current token
        let token = self.current_token.clone();
        match token {
            Token::Add => {
                let _capture = self.get_next_token();
                let r_expr = self.generate_ast()?;
                Ok(Node::Add(Box::new(l_expr), Box::new(r_expr)))
            }
            Token::Multiply => {
                let _capture = self.get_next_token();
                let r_expr = self.generate_ast()?;
                Ok(Node::Multiply(Box::new(l_expr), Box::new(r_expr)))
            },
            Token::Divide => {
                let _capture = self.get_next_token();
                let r_expr = self.generate_ast()?;
                Ok(Node::Divide(Box::new(l_expr), Box::new(r_expr)))
            },
            _ => return Err(ParseError::InvalidOperator("Bad token".to_string()))
        }
    }
}

pub enum ParseError {
    InvalidOperator(String)
}

