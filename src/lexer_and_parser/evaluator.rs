
pub fn evaluate(expr: Node) -> Result<EvalResult, Box<dyn error::Error>> {
    match expr {
        Node::NegativeNumberExpression(_) | Node::NumberExpression(_) => Ok(EvalResult::Number(evaluate_numerics(expr)?)),
        Node::BinaryExpr{ .. } => Ok(EvalResult::Number(evaluate_numerics(expr)?)),
        Node::AssignmentExpression { identifier, assignment_operator, expr } => {
            Ok(evaluate_assignments(*identifier, assignment_operator, *expr)?)
        },
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

fn evaluate_assignments(identifier: Node, assignment_operator: Token, expr: Node) -> Result<EvalResult, Box<dyn error::Error>> {
   // Just experimental, so we'll assume that every assignment goes with '='
   // Suppot for -= / += will be added later.
   // Evaluate right hand expression
   let value = evaluate(expr)?;
   let identifier_str = match identifier {
       Node::LiteralExpression(val) => EvalResult::Literal(val),
       _ => return Err("couldnt evaluate".into())
   };
   Ok(EvalResult::Assignment { identifier: Box::new(identifier_str), value: Box::new(value) }) 

}

