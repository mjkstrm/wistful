// Standard libraries
use std::iter::Peekable;
use std::str::Chars;

// Internal modules
use super::token::{Keyword, Token};

// Tokenizer
/*
    Lifetime annotation <'a> makes sure,
    that the field "expr" will outlive the borrower.

    in a nutshell: You can only keep using tokenizer aslong as the input (expr) is valid.

    Any input which is borrowed (expression in this case), must outlive (or live as long as) the borrower!
*/
pub struct Tokenizer<'a> {
    // Given expression
    /*
        Peekable iterator type allows us to use .next() and .peek() methods.
        .next() returns the next character in the given input, and consumes it.
        .peek() return the next character in the give input without consuming it.
    */
    pub expr: Peekable<Chars<'a>>,
}

// Implementation of constructing a new instance of Tokenizer
// impl<'a> declares the lifetime 'a
// Tokenizer<'a> uses the lifetime 'a
impl<'a> Tokenizer<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        Tokenizer {
            expr: new_expr.chars().peekable(),
        }
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
                    } else if next_char == &'(' {
                        return None;
                    } else {
                        break;
                    }
                }
                Some(Token::Num(number.parse::<f64>().unwrap()))
            }
            Some('a'..='z') => {
                let mut characters = next_char?.to_string();
                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_whitespace() {
                        break;
                    } else {
                        characters.push(self.expr.next()?);
                    }
                }
                match characters.as_str() {
                    "true" => {
                        return Some(Token::Literal {
                            literal: characters,
                            keyword: Keyword::True,
                        });
                    }
                    "false" => {
                        return Some(Token::Literal {
                            literal: characters,
                            keyword: Keyword::False,
                        });
                    }
                    "if" => {
                        return Some(Token::Literal {
                            literal: characters,
                            keyword: Keyword::IF,
                        });
                    }
                    "endif" => {
                        return Some(Token::Literal {
                            literal: characters,
                            keyword: Keyword::ENDIF,
                        });
                    }
                    "else" => {
                        return Some(Token::Literal {
                            literal: characters,
                            keyword: Keyword::ELSE,
                        });
                    }
                    "elif" => {
                        return Some(Token::Literal {
                            literal: characters,
                            keyword: Keyword::ELIF,
                        });
                    }
                    "for" => {
                        return Some(Token::Literal {
                            literal: characters,
                            keyword: Keyword::FOR,
                        })
                    }
                    // Rust retardness :D
                    _ => return Some(Token::Identifier(characters)),
                };
            }
            Some('"') => {
                let mut characters = String::new();
                while let Some(next_char) = self.expr.peek() {
                    if next_char == &'"' {
                        break;
                    } else {
                        characters.push(self.expr.next()?);
                    }
                }
                Some(Token::Literal {
                    literal: characters,
                    keyword: Keyword::None,
                })
            }
            // Operators
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Pow),
            Some('=') => {
                if self.expr.peek() == Some(&'=') {
                    self.expr.next()?;
                    Some(Token::Equals)
                } else {
                    Some(Token::Assignment)
                }
            }
            // Parentheses
            Some('(') => Some(Token::LeftParenthese),
            Some(')') => Some(Token::RightParenthese),
            // Braces
            Some('{') => Some(Token::LeftBrace),
            Some('}') => Some(Token::RightBrace),
            // Whitespace
            //c if c?.is_whitespace() => Some(Token::Whitespace),
            Some(' ') => Some(Token::Whitespace),
            // EOF
            Some('\n') => Some(Token::Whitespace),
            Some('\r') => Some(Token::Whitespace),
            // Tab
            Some('\t') => Some(Token::Whitespace),
            // Null
            Some('\0') => Some(Token::EOF),
            None => Some(Token::EOF),
            Some(_) => None,
        }
    }
}
