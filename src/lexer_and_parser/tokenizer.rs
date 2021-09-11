// Standard libraries
use std::iter::Peekable;
use std::str::Chars;
// Internal modules
use super::token::Token;
// Tokenizer
/*
    Lifetime annotation <'a> makes sure,
    that the field "expr" will outlive the borrower.

    Any input which is borrowed (expression in this case), must outlive the borrower!
*/
pub struct Tokenizer<'a> {
    // Given expression
    /*
        Peekable iterator type allows us to use .next() and .peek() methods.
        .next() returns the next character in the given input, and consumes it.
        .peek() return the next character in the give input without consuming it.
    */
    pub expr: Peekable<Chars<'a>>
}

// Implementation of constructing a new instance of Tokenizer
// impl<'a> declares the lifetime 'a
// Tokenizer<'a> uses the lifetime 'a
impl<'a> Tokenizer<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        println!("{:p}", new_expr); // <- Prints out Y
        Tokenizer { expr: new_expr.chars().peekable(), }
    }
}

// Implementation of iterator trait for the Tokenizer
// Iterator trait only requires a method to be defined for the 'next' element.
impl<'a> Iterator for Tokenizer<'a> {
    // We can reference to this type with Self::Item
    type Item = Token;
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<Token> {
        let next_char = self.expr.next();
        // Match is basically the equilevant of switch.
        match next_char {
            // Same as "case"
            // Check if char is a number
            Some('0'..='9') => {
                let mut number = next_char?.to_string();
                /*
                    Numbers need a bit more extra processing,
                    since we can be dealing with decimals aswell.

                    So we iterate throught the coming characters, and check
                    whether they are a another number or a decimal delimiter.

                    If the next characters are numbers or a delimiter -> concat values into a
                    one number
                */
                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_numeric() || next_char == &'.' {
                        number.push(self.expr.next()?);
                    }
                    else if next_char == &'(' {
                        return None;
                    }
                    else {
                        break;
                    }
                }
                Some(Token::Num(number.parse::<f64>().unwrap()))
            },
            Some('a'..='z') => {
                let mut characters = next_char?.to_string();
                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_whitespace() {
                        break;
                    }
                    else {
                        characters.push(self.expr.next()?);
                    }
                }
                Some(Token::Literal(characters))
            }
            // Operators
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Pow),
            Some('=') => Some(Token::Assignment),
            // Parentheses
            Some('(') => Some(Token::LeftParenthese),
            Some(')') => Some(Token::RightParenthese),
            // Whitespace
            //c if c?.is_whitespace() => Some(Token::Whitespace),
            Some(' ') => Some(Token::Whitespace),
            // End of file
            Some('\n') => Some(Token::EOF),
            None => Some(Token::EOF),
            Some(_) => None,
        }
    }
}
