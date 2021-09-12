// Libraries
use std::error;
use std::fmt;
// Internal modules
use super::ast::Node;
use super::token::Token;

pub struct Evaluator {
    ast: Node,
    // Storing evaluated variables
    pub variable_storage: Vec<Variable>
}

// Store variables in a global vector
#[derive(Debug)]
pub struct Variable {
    // Fields set to public just for debugging purposes
    pub name: String,
    pub value: f64
}

// Enum holding the evaluate_numerics values
#[derive(Debug, Clone)]
pub enum EvalResult {
    Number(f64),
    Literal(String),
    Assignment {
        identifier: Box<EvalResult>,
        value: Box<EvalResult>
    }
}

impl fmt::Display for EvalResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{:?}", self) 
    }
}



// Public methods
impl Evaluator {
    pub fn new(expr: Node) -> Self {
        Evaluator { ast: expr, variable_storage: Vec::new() }
    }

    // Start evaluating
    pub fn start_evaluating(&mut self) -> Result<(), Box<dyn error::Error>> {
        let expr : Node = self.ast.clone();
        match self.evaluate(expr) {
            Ok(..) => Ok(()),
            Err(e) => return Err(e.into()),
        }  
    }

    // Evaluate given node and return an EvalResult.
    pub fn evaluate(&mut self, expr: Node) -> Result<EvalResult, Box<dyn error::Error>> {
        match expr {
            Node::NegativeNumberExpression(_) | Node::NumberExpression(_) => Ok(EvalResult::Number(self.evaluate_numerics(expr)?)),
            Node::BinaryExpr{ .. } => Ok(EvalResult::Number(self.evaluate_numerics(expr)?)),
            Node::AssignmentExpression { identifier, assignment_operator, expr } => {
                Ok(self.evaluate_assignments(*identifier, assignment_operator, *expr)?)
            },
            // TODO: Handle variables
            _ => Err("Couldnt evaluate".into())
        }


    } 
}

// Private methods
impl Evaluator {
    // Evaluating numeric values and arithmetics
    fn evaluate_numerics(&mut self, expr: Node) -> Result<f64, Box<dyn error::Error>> {
        use self::Node::*;
        match expr {
            NumberExpression(f) => Ok(f),
            NegativeNumberExpression(f) => Ok(-self.evaluate_numerics(*f)?),
            BinaryExpr { l_expr, operator, r_expr } => {
                match operator {
                    Token::Add => Ok(self.evaluate_numerics(*l_expr)? + self.evaluate_numerics(*r_expr)?),
                    Token::Multiply => Ok(self.evaluate_numerics(*l_expr)? * self.evaluate_numerics(*r_expr)?),
                    Token::Divide => Ok(self.evaluate_numerics(*l_expr)? / self.evaluate_numerics(*r_expr)?),
                    Token::Pow => Ok(self.evaluate_numerics(*l_expr)?.powf(self.evaluate_numerics(*r_expr)?)),
                    // Fix this, bad implementation
                    _ => Err("Couldnt evaluate".into())
                }
            }
            _ => Err("Not implemented.".into())
        }
    }

    fn evaluate_assignments(&mut self, identifier: Node, assignment_operator: Token, expr: Node) -> Result<EvalResult, Box<dyn error::Error>> {
       // Just experimental, so we'll assume that every assignment goes with '='
       // Suppot for -= / += will be added later.
       // Evaluate right hand expression
       let value = self.evaluate(expr)?;
       // Initialize an empty string for variable name.
       let mut variable_name = String::new();
       // Set identifier
       let identifier_str = match identifier {
           Node::LiteralExpression(val) => {
               variable_name = val.clone();
               EvalResult::Literal(val)},
           _ => return Err("couldnt evaluate".into())
       };
       // Depending on the assigned values type, create a properly typed variable
       // Numbers
       if let EvalResult::Number(f) = value {
            // TODO: If variable already exists, last value. Types can also be changed @ runtime
            // Create a new variable
            let new_variable = Variable {
                name: variable_name,
                value: f
            };
            self.variable_storage.push(new_variable);
       }

       Ok(EvalResult::Assignment { identifier: Box::new(identifier_str), value: Box::new(value) }) 
    }
}
