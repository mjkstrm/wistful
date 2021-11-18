// Libraries
use std::collections::HashMap;
use std::error;
use std::fmt;
// Internal modules
use super::ast::Node;
use super::token::{Keyword, Token};

pub struct Evaluator {
    pub ast: Option<Node>,
    // Storing evaluated variables
    pub variable_storage: HashMap<String, VariableValue>,
}
// TODO: Move to a separate file which contains helper classes/methods.
// TODO: struct -> enum, different variants for different types. i.e string, float, int etc.
#[derive(Debug)]
pub enum VariableValue {
    // Fields set to public just for debugging purposes
    Number(f64),
    Literal(String),
    Boolean(bool),
}

// Actual result of the expression evaluating.
#[derive(Debug, Clone, PartialEq)]
pub enum EvalResult {
    Number(f64),
    Literal(String),
    Boolean(bool),
    Assignment {
        identifier: Box<EvalResult>,
        value: Box<EvalResult>,
    },
    EmptyResult,
}
// Display trait for EvalResult. Used to parse values for variable instantiating and debugging.
impl fmt::Display for EvalResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Public methods
impl Evaluator {
    // Instantiate a new evaluator object
    /*
        Given expression is taken as an Option.
        It grants us the same freedom as optional parameters in other languages gives.

        Evaluator gets instantiated before actual parsing begins, so we're missing an actual AST at that point in time.
        Variable storage will be eventually removed from evaluator.
    */
    pub fn new(expr: Option<Node>) -> Self {
        Evaluator {
            ast: expr,
            variable_storage: HashMap::new(),
        }
    }
    // Start evaluating
    pub fn start_evaluating(&mut self) -> Result<(), Box<dyn error::Error>> {
        let expr: Node = self.ast.clone().unwrap();
        match self.evaluate(expr) {
            Ok(r) => {
                println!("EVALUATOR: {:?}", r);
                Ok(())
            }
            Err(e) => return Err(e.into()),
        }
    }
}

// Private methods
impl Evaluator {
    // Evaluate given node and return an EvalResult.
    fn evaluate(&mut self, expr: Node) -> Result<EvalResult, Box<dyn error::Error>> {
        match expr {
            Node::NegativeNumberExpression(_) | Node::NumberExpression(_) => {
                Ok(EvalResult::Number(self.evaluate_numerics(expr)?))
            }
            Node::BinaryExpr { .. } => Ok(EvalResult::Number(self.evaluate_numerics(expr)?)),
            Node::AssignmentExpression {
                identifier,
                assignment_operator,
                expr,
            } => Ok(self.evaluate_assignments(*identifier, assignment_operator, *expr)?),
            // Literals/keywords
            Node::LiteralExpression(string, keyword) => {
                // Handle keywords
                match keyword {
                    Keyword::True => return Ok(EvalResult::Boolean(true)),
                    Keyword::False => return Ok(EvalResult::Boolean(false)),
                    _ => return Ok(EvalResult::Literal(string)),
                };
            }
            // Handle variables
            Node::IdentifierExpression(identifier) => {
                // Get value from storage
                let value = self.variable_storage.get(&identifier);
                match value {
                    Some(VariableValue::Boolean(b)) => Ok(EvalResult::Boolean(*b)),
                    Some(VariableValue::Number(f)) => Ok(EvalResult::Number(*f)),
                    Some(VariableValue::Literal(s)) => Ok(EvalResult::Literal(s.to_string())),
                    None => Err("Could not find a variable with given identifier".into()),
                }
            }
            // Handle If expressions
            Node::IfExpression {
                condition,
                then_branch,
                else_branch,
            } => {
                // Evaluate whether the condition is true or false.
                // If true, evaluate "then_branch"
                // If false, evaluate nothing (until support for else branch is added)

                Ok(self.evaluate_if_expression(*condition, *then_branch, *else_branch)?)
            }
            _ => Err("Couldnt evaluate".into()),
        }
    }

    // Handle numerics and binary expressions
    fn evaluate_numerics(&mut self, expr: Node) -> Result<f64, Box<dyn error::Error>> {
        use self::Node::*;
        match expr {
            NumberExpression(f) => Ok(f),
            NegativeNumberExpression(f) => Ok(-self.evaluate_numerics(*f)?),
            BinaryExpr {
                l_expr,
                operator,
                r_expr,
            } => {
                match operator {
                    Token::Add => {
                        Ok(self.evaluate_numerics(*l_expr)? + self.evaluate_numerics(*r_expr)?)
                    }
                    Token::Subtract => {
                        Ok(self.evaluate_numerics(*l_expr)? - self.evaluate_numerics(*r_expr)?)
                    }
                    Token::Multiply => {
                        Ok(self.evaluate_numerics(*l_expr)? * self.evaluate_numerics(*r_expr)?)
                    }
                    Token::Divide => {
                        Ok(self.evaluate_numerics(*l_expr)? / self.evaluate_numerics(*r_expr)?)
                    }
                    Token::Pow => Ok(self
                        .evaluate_numerics(*l_expr)?
                        .powf(self.evaluate_numerics(*r_expr)?)),
                    // Fix this, bad implementation
                    _ => Err("Could not evaluate Binary Expression".into()),
                }
            }
            IdentifierExpression(identifier) => {
                let value = self.variable_storage.get(&identifier);
                match value {
                    Some(VariableValue::Number(f)) => Ok(*f),
                    _ => Err("Variable not found".into()),
                }
            }
            _ => Err("Not implemented.".into()),
        }
    }
    // Evaluate assignment expressions.
    fn evaluate_assignments(
        &mut self,
        identifier: Node,
        assignment_operator: Token,
        expr: Node,
    ) -> Result<EvalResult, Box<dyn error::Error>> {
        // Just experimental, so we'll assume that every assignment goes with '='
        // Suppot for -= / += will be added later.
        // Evaluate right hand expression
        let value = self.evaluate(expr)?;
        // Initialize an empty string for variable name.
        let mut variable_name = String::new();
        // Set identifier
        let identifier_str = match identifier {
            Node::IdentifierExpression(val) => {
                variable_name = val.clone();
                EvalResult::Literal(val)
            }
            _ => return Err("couldnt evaluate".into()),
        };
        // Depending on the assigned values type, create a properly typed variable
        match value.clone() {
            // Floats
            EvalResult::Number(f) => {
                let new_var = VariableValue::Number(f);
                self.variable_storage.insert(variable_name, new_var);
            }
            // Strings
            EvalResult::Literal(string) => {
                let new_var = VariableValue::Literal(string);
                self.variable_storage.insert(variable_name, new_var);
            }
            // Booleans
            EvalResult::Boolean(boolean) => {
                let new_var = VariableValue::Boolean(boolean);
                self.variable_storage.insert(variable_name, new_var);
            }
            _ => println!("Could not assign {0} to {1}", value, variable_name),
        }

        Ok(EvalResult::Assignment {
            identifier: Box::new(identifier_str),
            value: Box::new(value),
        })
    }
    // Evaluate blocks
    fn evaluate_if_expression(
        &mut self,
        condition: Option<Node>,
        then_branch: Vec<Node>,
        else_branch: Option<Node>,
    ) -> Result<EvalResult, Box<dyn error::Error>> {
        //
        //
        match condition {
            Some(Node::ConditionExpression {
                l_expr,
                operator,
                r_expr,
            }) => {
                // Compare each evaluated value. Both type and value is checked here.
                if self.evaluate(*l_expr)? == self.evaluate(*r_expr)? {
                    // Evaluate block
                    for expression in then_branch {
                        self.ast = Some(expression);
                        self.start_evaluating();
                    }
                } else {
                    match else_branch {
                        Some(branch) => {
                            if let Node::ElseExpression {
                                condition,
                                then_branch,
                                else_branch,
                            } = branch
                            {
                                self.evaluate_if_expression(*condition, *then_branch, *else_branch);
                            }
                        }
                        None => { /* Nothing to do if branch was not found */ }
                    }
                }
            }
            // Else clauses
            None => {
                for expression in then_branch {
                    self.ast = Some(expression);
                    self.start_evaluating();
                }
            }
            _ => {
               // Nothing to do 
            }
        }
        Ok(EvalResult::EmptyResult)
    }
    /*
    // Evaluate block statements (Then, else, function blocks etc.)
    // Go through each expression in given branch
    fn evaluate_block_statements(&mut self, block_statement: Vec<Node>) -> Result<EvalResult, Box<dyn error::Error>> {

    }*/
}
