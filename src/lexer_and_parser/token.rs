#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Assignment,
    Add,
    Subtract,
    Equals,
    Multiply,
    Divide,
    Pow,
    LeftParenthese,
    RightParenthese,
    Num(f64),
    Literal { literal: String, keyword: Keyword },
    Identifier(String),
    Whitespace,
    EOF,
    LeftBrace,
    RightBrace,
    GreaterThan,
    LessThan
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    None,
    True,
    False,
    IF,
    ENDIF,
    ELSE,
    ELIF,
    WHILE,
    BREAK,
}

// Arithmetic precedences
#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Default,
    // 0,
    AddAndSubtract,
    MultiplyAndDivide,
    Power,
    NegativeValue,
}

// Function to get operator precedence
impl Token {
    pub fn get_precedence(&self) -> Precedence {
        use self::Precedence::*;
        use self::Token::*;
        // What to return
        match *self {
            Add | Subtract => AddAndSubtract,
            Multiply | Divide => MultiplyAndDivide,
            Pow => Power,
            _ => Default,
        }
    }
}
