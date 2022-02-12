use std::fmt;
use crate::Node::ConditionExpression;

// Internal uses
use super::ast::Node;
use super::token::{Keyword, Precedence, Token};
use super::tokenizer::Tokenizer;

pub struct Parser<'a> {
    // input to be parsed
    pub tokenizer: Tokenizer<'a>,
    current_token: Token,
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
        })
    }

    // Method in the public interface for parsing the expression
    pub fn parse(&mut self) -> Result<Vec<Node>, ParseError> {
        while self.current_token == Token::Whitespace {
            self.get_next_token()?;
        }
        let mut nodes = Vec::new();
        while self.current_token != Token::EOF {
            let ast = self.generate_ast(Precedence::Default)?;
            nodes.push(ast);
        }
        Ok(nodes)
    }
}

// Private methods
impl<'a> Parser<'a> {
    // Move on to the next token to be parsed.
    fn get_next_token(&mut self) -> Result<(), ParseError> {
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        if next_token == Token::Whitespace {
            self.get_next_token()?;
        } else {
            self.current_token = next_token;
        }
        Ok(())
    }

    fn generate_ast(&mut self, precedence: Precedence) -> Result<Node, ParseError> {
        if self.current_token == Token::EOF {
            return Ok(Node::EOF("EOF".to_string()));
        }
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
                // Check token to know if we're supposed to return an conditional expression
                if self.check_token(Token::Equals)? {
                    let r_expr = self.generate_ast(Precedence::Default)?;
                    return Ok(ConditionExpression {
                        l_expr: Box::new(Node::NumberExpression(i)),
                        operator: Token::Equals,
                        r_expr: Box::new(r_expr)
                    })
                }
                Ok(Node::NumberExpression(i))
            }
            Token::LeftParenthese => {
                self.get_next_token()?;
                let l_expr = self.generate_ast(Precedence::Default)?;
                // Make sure there is a pair for the opening parenthese. If not, return an error.
                self.check_paren(Token::RightParenthese)?;

                Ok(l_expr)
            }
            Token::Identifier(string) => {
                self.get_next_token()?;
                // Expecting an assignment after identifier
                if self.check_token(Token::Assignment)? {
                    let r_expr = self.generate_ast(Precedence::Default)?;
                    let id_expr = Node::IdentifierExpression(string);
                    return Ok(Node::AssignmentExpression {
                        identifier: Box::new(id_expr),
                        assignment_operator: Token::Assignment,
                        expr: Box::new(r_expr),
                    });
                    // TODO: Instead of checking if token is an 'equals' token, check for any comparison tokens.
                } else if self.check_token(Token::Equals)? {
                    let r_expr = self.generate_ast(Precedence::Default)?;
                    let id_expr = Node::IdentifierExpression(string);
                    //
                    return Ok(Node::ConditionExpression {
                        l_expr: Box::new(id_expr),
                        operator: Token::Equals,
                        r_expr: Box::new(r_expr),
                    });
                } else {
                    return Ok(Node::IdentifierExpression(string));
                }
            }
            Token::Literal { literal, keyword } => {
                self.get_next_token()?;
                // Handle if clause
                if keyword == Keyword::IF {
                    return self.parse_if_expression();
                }
                // Parse ELSE and ELIF branches
                else if keyword == Keyword::ELSE || keyword == Keyword::ELIF {
                    return self.parse_else_expression(keyword);
                }
                // For loop
                else if keyword == Keyword::WHILE {
                    return self.parse_while_expression();
                }

                return Ok(Node::LiteralExpression(literal, keyword));
            }
            _ => {
                return Err(ParseError::UnableToParse(format!(
                    "Could not parse Token: {0:?}",
                    self.current_token
                )));
            }
        }
    }

    // Parse If, Else, Else if.
    fn parse_if_expression(&mut self) -> Result<Node, ParseError> {
        // Handle if clause
        // Parse condition for THEN branch
        let condition = Some(self.generate_ast(Precedence::Default)?);
        // If we're missing an opening brace for the branch, return error.
        if !self.check_token(Token::LeftBrace)? {
            return Err(ParseError::UnableToParse(format!(
                "Missing opening brace for {0:?}",
                condition
            )));
        }
        // Generate nodes for the branch until ENDIF or ELSE is reached
        let mut else_branch = None;
        let mut then_branch = Vec::new();
        // If closing branch is found, set this flag variable to true, and check
        // whether else clause is to be parsed.
        let mut check_for_else = false;
        loop {
            // If eof has been reached without closing brace, return error.
            if self.current_token == Token::EOF {
                return Err(ParseError::UnableToParse(format!(
                    "Missing closing brace for {0:?}",
                    condition
                )));
            }
            let branch_for_then = self.generate_ast(Precedence::Default)?;
            match branch_for_then {
                Node::ElseExpression {
                    condition: _,
                    then_branch: _,
                    else_branch: _,
                } => {
                    else_branch = Some(branch_for_then);
                    break;
                }
                _ => {
                    if !check_for_else {
                        then_branch.push(branch_for_then);
                        // If closing brace is found, stop iterating.
                        if self.check_token(Token::RightBrace)? {
                            check_for_else = true;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        return Ok(Node::IfExpression {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        });
    }
    // Parse else/elif expressions
    fn parse_else_expression(&mut self, keyword: Keyword) -> Result<Node, ParseError> {
        // Initialize condition as None, since condition is not mandatory for an else
        // clause
        let mut condition: Option<Node> = None;
        // Whether we have a condition to evaluate or not.
        if keyword == Keyword::ELIF {
            condition = Some(self.generate_ast(Precedence::Default)?);
        }
        // If we're missing an opening brace for the branch, return error.
        if !self.check_token(Token::LeftBrace)? {
            return Err(ParseError::UnableToParse(format!(
                "Missing opening brace for {0:?}",
                condition
            )));
        }
        // Generate a statement
        let mut else_branch = None;
        let mut then_branch = Vec::new();
        // If closing branch is found, set this flag variable to true, and check
        // whether else clause is to be parsed
        let mut check_for_else = false;
        loop {
            // If eof has been reached without closing brace, return error.
            if self.current_token == Token::EOF {
                if condition.is_some() {
                    return Err(ParseError::UnableToParse(format!(
                        "Missing closing brace for {0:?}",
                        condition
                    )))
                }
                else {
                    return Err(ParseError::UnableToParse("Missing closing brace for Else".to_string()))
                }
            }
            let branch_for_then = self.generate_ast(Precedence::Default)?;
            match branch_for_then {
                Node::ElseExpression {
                    condition: _,
                    then_branch: _,
                    else_branch: _,
                } => {
                    else_branch = Some(branch_for_then);
                    break;
                }
                _ => {
                    if !check_for_else {
                        then_branch.push(branch_for_then);
                        // If closing brace is found, stop iterating.
                        if self.check_token(Token::RightBrace)? {
                            check_for_else = true;
                            // If there is no condition, break out, we know that wont
                            // be another else in the tree
                            if keyword == Keyword::ELSE {
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        return Ok(Node::ElseExpression {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        });
    }

    fn parse_while_expression(&mut self) -> Result<Node, ParseError> {
        // Get iteration condition
        let mut condition: Option<Node> = None;
        // If next token is opening brace, skip trying to parse an condition for the iteration
        if !self.check_token(Token::LeftBrace)? {
            condition = Some(self.generate_ast(Precedence::Default)?);
        }
        // If opening brace is not found, return an error
        if !self.check_token(Token::LeftBrace)? {
            return Err(ParseError::UnableToParse(format!(
                "Missing opening brace for {0:?}",
                condition
            )))
        }
        let mut then_branch = Vec::new();
        // Iterate until closing brace is found.
        loop {
            if self.check_token(Token::RightBrace)? {
                break;
            }
            let branch_for_then = self.generate_ast(Precedence::Default)?;
            then_branch.push(branch_for_then);
            // If eof is reached and closing brace is not found, return error.
            if self.current_token == Token::EOF {
                if condition.is_some() {
                    return Err(ParseError::UnableToParse(format!(
                        "Missing closing brace for {0:?}",
                        condition
                    )))
                }
                else {
                    return Err(ParseError::UnableToParse("Missing closing brace for While expression.".to_string()))
                }
            }
        }
        return Ok(Node::WhileExpression {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch)
        });
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

    fn check_token(&mut self, expected_token: Token) -> Result<bool, ParseError> {
        if expected_token == self.current_token {
            self.get_next_token()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn parse_binary_expression(&mut self, l_expr: Node) -> Result<Node, ParseError> {
        // Clone our current token
        let token = self.current_token.clone();
        match token {
            Token::Add => {
                let _capture = self.get_next_token()?;
                let r_expr = self.generate_ast(Precedence::AddAndSubtract)?;
                Ok(Node::BinaryExpr {
                    l_expr: Box::new(l_expr),
                    operator: Token::Add,
                    r_expr: Box::new(r_expr),
                })
            }
            Token::Subtract => {
                let _capture = self.get_next_token()?;
                let r_expr = self.generate_ast(Precedence::AddAndSubtract)?;
                Ok(Node::BinaryExpr {
                    l_expr: Box::new(l_expr),
                    operator: Token::Subtract,
                    r_expr: Box::new(r_expr),
                })
            }
            Token::Multiply => {
                let _capture = self.get_next_token()?;
                let r_expr = self.generate_ast(Precedence::MultiplyAndDivide)?;
                Ok(Node::BinaryExpr {
                    l_expr: Box::new(l_expr),
                    operator: Token::Multiply,
                    r_expr: Box::new(r_expr),
                })
            }
            Token::Divide => {
                let _capture = self.get_next_token()?;
                let r_expr = self.generate_ast(Precedence::MultiplyAndDivide)?;
                Ok(Node::BinaryExpr {
                    l_expr: Box::new(l_expr),
                    operator: Token::Divide,
                    r_expr: Box::new(r_expr),
                })
            }
            Token::Pow => {
                let _capture = self.get_next_token()?;
                let r_expr = self.generate_ast(Precedence::Power)?;
                Ok(Node::BinaryExpr {
                    l_expr: Box::new(l_expr),
                    operator: Token::Pow,
                    r_expr: Box::new(r_expr),
                })
            }
            _ => return Err(ParseError::InvalidOperator("Bad token".to_string())),
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
    UnableToParse(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            self::ParseError::UnableToParse(e) => write!(f, "{}", e),
            self::ParseError::InvalidOperator(e) => write!(f, "{}", e),
        }
    }
}
