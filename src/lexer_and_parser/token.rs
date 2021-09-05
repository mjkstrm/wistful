
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Assignment,
    Add,
    Subtract,
    Multiply,
    Divide,
    Pow,
    LeftParenthese,
    RightParenthese,
    Num(f64),
    Literal(String),
    Whitespace,
    EOF
}

// Arithmetic precedences
#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Default, // 0,
    AddAndSubtract,
    MultiplyAndDivide,
    Power,
    NegativeValue
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
            _ => Default
        }
    }
}

