use std::fmt;
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
            self.get_next_token();
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
                Ok(Node::NumberExpression(i))
            }
            Token::LeftParenthese => {
                self.get_next_token()?;
                let l_expr = self.generate_ast(Precedence::Default)?;
                self.check_paren(Token::RightParenthese)?;

                if self.current_token == Token::LeftParenthese {
                    let r_expr = self.generate_ast(Precedence::MultiplyAndDivide)?;
                    return Ok(Node::BinaryExpr {
                        l_expr: Box::new(l_expr),
                        operator: Token::Multiply,
                        r_expr: Box::new(r_expr),
                    });
                }

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
                // If clause, TODO: Comment this shit out properly
                if keyword == Keyword::IF { 
                    let condition = Some(self.generate_ast(Precedence::Default)?);
                    if !self.check_token(Token::LeftBrace)? {
                        return Err(ParseError::UnableToParse(format!("Missing opening brace for {0:?}", condition))) 
                    }
                    // Generate nodes for the branch until ENDIF or ELSE is reached
                    let mut else_branch = None;
                    let mut then_branch = Vec::new();
                    let mut check_for_else = false;
                    while let branch_for_then = self.generate_ast(Precedence::Default)? {
                        // If closing brace is found, stop iterating.
                        if self.check_token(Token::RightBrace)? {
                            check_for_else = true;
                            break;
                        }
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
                                }
                                else {
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
                // Parse ELSE and ELIF branches
                if keyword == Keyword::ELSE || keyword == Keyword::ELIF {
                    let mut condition: Option<Node> = None;
                    // Whether we have a condition to evaluate or not.
                    if keyword == Keyword::ELIF {
                        condition = Some(self.generate_ast(Precedence::Default)?);
                    }
                    if !self.check_token(Token::LeftBrace)? {
                        return Err(ParseError::UnableToParse(format!("Missing opening brace for {0:?}", condition))) 
                    }
                    // Generate a statement
                    let mut else_branch = None;
                    let mut then_branch = Vec::new();
                    let mut check_for_else = false;
                    while let branch_for_then = self.generate_ast(Precedence::Default)? {
                        // If closing brace is found, stop iterating.
                        if self.check_token(Token::RightBrace)? {
                            check_for_else = true;
                            break; 
                        }
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
                                }
                                else {
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
                return Ok(Node::LiteralExpression(literal, keyword));
            }
            _ => return Err(ParseError::InvalidOperator("Bad start".to_string())),
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
                let _capture = self.get_next_token();
                let r_expr = self.generate_ast(Precedence::AddAndSubtract)?;
                Ok(Node::BinaryExpr {
                    l_expr: Box::new(l_expr),
                    operator: Token::Add,
                    r_expr: Box::new(r_expr),
                })
            }
            Token::Subtract => {
                let _capture = self.get_next_token();
                let r_expr = self.generate_ast(Precedence::AddAndSubtract)?;
                Ok(Node::BinaryExpr {
                    l_expr: Box::new(l_expr),
                    operator: Token::Subtract,
                    r_expr: Box::new(r_expr),
                })
            }
            Token::Multiply => {
                let _capture = self.get_next_token();
                let r_expr = self.generate_ast(Precedence::MultiplyAndDivide)?;
                Ok(Node::BinaryExpr {
                    l_expr: Box::new(l_expr),
                    operator: Token::Multiply,
                    r_expr: Box::new(r_expr),
                })
            }
            Token::Divide => {
                let _capture = self.get_next_token();
                let r_expr = self.generate_ast(Precedence::MultiplyAndDivide)?;
                Ok(Node::BinaryExpr {
                    l_expr: Box::new(l_expr),
                    operator: Token::Divide,
                    r_expr: Box::new(r_expr),
                })
            }
            Token::Pow => {
                let _capture = self.get_next_token();
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


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_binary_expressions() {
        let mut parser = Parser::new("
            1+2
            1-1
            2*2
            10/2
            5^2
            x + 2
            ").unwrap();
        let add = Node::BinaryExpr { l_expr: Box::new(Node::NumberExpression(1.0)), operator: Token::Add, r_expr: Box::new(Node::NumberExpression(2.0)) };
        let subtract = Node::BinaryExpr { l_expr: Box::new(Node::NumberExpression(1.0)), operator: Token::Subtract, r_expr: Box::new(Node::NumberExpression(1.0))};
        let multiply = Node::BinaryExpr { l_expr: Box::new(Node::NumberExpression(2.0)), operator: Token::Multiply, r_expr: Box::new(Node::NumberExpression(2.0))};
        let divide = Node::BinaryExpr { l_expr: Box::new(Node::NumberExpression(10.0)), operator: Token::Divide, r_expr: Box::new(Node::NumberExpression(2.0))};
        let pow = Node::BinaryExpr { l_expr: Box::new(Node::NumberExpression(5.0)), operator: Token::Pow, r_expr: Box::new(Node::NumberExpression(2.0))};
        let add_to_variable = Node::BinaryExpr { l_expr: Box::new(Node::IdentifierExpression("x".to_string())), operator: Token::Add, r_expr: Box::new(Node::NumberExpression(2.0))};

        let mut expected_expressions : Vec<Node> = Vec::new();
        expected_expressions.push(add);
        expected_expressions.push(subtract);
        expected_expressions.push(multiply);
        expected_expressions.push(divide);
        expected_expressions.push(pow);
        expected_expressions.push(add_to_variable);
        
        assert_eq!(parser.parse().unwrap(), expected_expressions);
    } 
    #[test]
    fn test_identifier_expression() {
        let mut parser = Parser::new("x").unwrap();
        let expected = Node::IdentifierExpression("x".to_string());

        assert_eq!(parser.parse().unwrap()[0], expected);
    }
    #[test]
    fn test_assignment_expression() {
        let mut parser = Parser::new("x = 5").unwrap();
        let expected = Node::AssignmentExpression { 
            identifier: Box::new(Node::IdentifierExpression("x".to_string())), assignment_operator: Token::Assignment, expr: Box::new(Node::NumberExpression(5.0))
        };
        assert_eq!(parser.parse().unwrap()[0], expected);
    }
}


