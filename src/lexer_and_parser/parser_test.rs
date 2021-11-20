// Different test for expressions which parser generates.
#[cfg(test)]
mod parser_test {
    use super::super::*;
    use crate::lexer_and_parser::ast::Node;
    use crate::lexer_and_parser::parser::Parser;
    use crate::lexer_and_parser::token::Token;
    #[test]
    fn test_binary_expressions() {
        let mut parser = Parser::new(
            "
            1+2
            1-1
            2*2
            10/2
            5^2
            x + 2
            ",
        )
        .unwrap();
        // 1 + 2
        let add = Node::BinaryExpr {
            l_expr: Box::new(Node::NumberExpression(1.0)),
            operator: Token::Add,
            r_expr: Box::new(Node::NumberExpression(2.0)),
        };
        // 1 - 1
        let subtract = Node::BinaryExpr {
            l_expr: Box::new(Node::NumberExpression(1.0)),
            operator: Token::Subtract,
            r_expr: Box::new(Node::NumberExpression(1.0)),
        };
        // 2 * 2
        let multiply = Node::BinaryExpr {
            l_expr: Box::new(Node::NumberExpression(2.0)),
            operator: Token::Multiply,
            r_expr: Box::new(Node::NumberExpression(2.0)),
        };
        // 10 / 2
        let divide = Node::BinaryExpr {
            l_expr: Box::new(Node::NumberExpression(10.0)),
            operator: Token::Divide,
            r_expr: Box::new(Node::NumberExpression(2.0)),
        };
        // 5 ^ 2
        let pow = Node::BinaryExpr {
            l_expr: Box::new(Node::NumberExpression(5.0)),
            operator: Token::Pow,
            r_expr: Box::new(Node::NumberExpression(2.0)),
        };
        // x + 2
        let add_to_variable = Node::BinaryExpr {
            l_expr: Box::new(Node::IdentifierExpression("x".to_string())),
            operator: Token::Add,
            r_expr: Box::new(Node::NumberExpression(2.0)),
        };
        // Add expected expressions to vector.
        let expected_expressions = vec![add, subtract, multiply, divide, pow, add_to_variable];

        assert_eq!(parser.parse().unwrap(), expected_expressions);
    }
    #[test]
    fn test_precedence() {
        let mut parser = Parser::new(
            "
            2+2*5
            (2+2)*5",
        )
        .unwrap();
        // 2 + 2 * 5
        let multi_precedence = Node::BinaryExpr {
            l_expr: Box::new(Node::NumberExpression(2.0)),
            operator: Token::Add,
            r_expr: Box::new(Node::BinaryExpr {
                l_expr: Box::new(Node::NumberExpression(2.0)),
                operator: Token::Multiply,
                r_expr: Box::new(Node::NumberExpression(5.0)),
            }),
        };
        // (2 + 2) * 5
        let parentheses_precedence = Node::BinaryExpr {
            l_expr: Box::new(Node::BinaryExpr {
                l_expr: Box::new(Node::NumberExpression(2.0)),
                operator: Token::Add,
                r_expr: Box::new(Node::NumberExpression(2.0)),
            }),
            operator: Token::Multiply,
            r_expr: Box::new(Node::NumberExpression(5.0)),
        };
        let mut expected_expressions = vec![multi_precedence, parentheses_precedence];
        assert_eq!(parser.parse().unwrap(), expected_expressions)
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
        // x = 5
        let expected = Node::AssignmentExpression {
            identifier: Box::new(Node::IdentifierExpression("x".to_string())),
            assignment_operator: Token::Assignment,
            expr: Box::new(Node::NumberExpression(5.0)),
        };
        assert_eq!(parser.parse().unwrap()[0], expected);
    }
}
