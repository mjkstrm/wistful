// Different test for expressions which parser generates.
#[cfg(test)]
mod parser_test {
    use crate::lexer_and_parser::ast::Node;
    use crate::lexer_and_parser::parser::Parser;
    use crate::lexer_and_parser::token::Token;
    use crate::lexer_and_parser::token::Token::Num;
    use crate::Node::{BinaryExpr, ConditionExpression, IdentifierExpression, NumberExpression};

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
        let expected_expressions = vec![multi_precedence, parentheses_precedence];
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

    #[test]
    fn test_if_expression() {
        let mut parser = Parser::new(

            "if x == 15 {
                x = 25
            }
            elif x == 10 {
                y = 10
                x = y + 2
            }
            else {
                x = 17
            }").unwrap();
        // Expected expression...
        // First form the conditions:
        // if condition - x == 15
        let if_condition = Box::new(Some(Node::ConditionExpression {
            l_expr: Box::new(Node::IdentifierExpression("x".to_string())),
            operator: Token::Equals,
            r_expr: Box::new(Node::NumberExpression(15.)),
        }));
        // then branch.. x = 25
        let then = Box::new(vec![Node::AssignmentExpression {
            identifier: Box::new(Node::IdentifierExpression("x".to_string())),
            assignment_operator: Token::Assignment,
            expr: Box::new(NumberExpression(25.)),
        }]);
        // elif condition - x == 10
        let elif_condition = Box::new(Some(Node::ConditionExpression {
            l_expr: Box::new(Node::IdentifierExpression("x".to_string())),
            operator: Token::Equals,
            r_expr: Box::new(Node::NumberExpression(10.0)),
        }));
        // elif then...
        // y = 10
        // x = y + 2
        let elif_then = Box::new(vec![
            Node::AssignmentExpression {
                identifier: Box::new(Node::IdentifierExpression("y".to_string())),
                assignment_operator: Token::Assignment,
                expr: Box::new(NumberExpression(10.)),
            },
            Node::AssignmentExpression {
                identifier: Box::new(Node::IdentifierExpression("x".to_string())),
                assignment_operator: Token::Assignment,
                expr: Box::new(Node::BinaryExpr {
                    l_expr: Box::new(IdentifierExpression("y".to_string())),
                    operator: Token::Add,
                    r_expr: Box::new(Node::NumberExpression(2.)),
                }),
            }]);
        // else then
        // x = 17
        let else_then = Box::new(vec![Node::AssignmentExpression {
            identifier: Box::new(Node::IdentifierExpression("x".to_string())),
            assignment_operator: Token::Assignment,
            expr: Box::new(NumberExpression(17.)),
        }]);

        // else if expression
        let elif_expression = Box::new(Some(Node::ElseExpression {
            condition: elif_condition,
            then_branch: elif_then,
            else_branch: Box::new(
                Some(Node::ElseExpression {
                    condition: Box::new(None),
                    then_branch: else_then,
                    else_branch: Box::new(None),
                })
            ),
        }));

        // Now that every neede piece is constructed, form the final if expression
        let if_expression = Node::IfExpression {
            condition: if_condition,
            then_branch: then,
            else_branch: elif_expression,
        };
        assert_eq!(parser.parse().unwrap()[0], if_expression);
    }

    #[test]
    fn test_while_expression() {
        let mut parser = Parser::new(
            "
            while 1 == 1 {
                1 + 1
            }"
        ).unwrap();
        let expected = Node::WhileExpression {
            condition: Box::new(Some(ConditionExpression {
                l_expr: Box::new(NumberExpression(1.)),
                operator: Token::Equals,
                r_expr: Box::new(NumberExpression(1.))
            })),
            then_branch: Box::new(vec![BinaryExpr {
                l_expr: Box::new(NumberExpression(1.)),
                operator: Token::Add,
                r_expr: Box::new(NumberExpression(1.))
            }])
        };
        assert_eq!(parser.parse().unwrap()[0], expected);
    }
}
