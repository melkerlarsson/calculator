/* use parser::Parser;
 */

use std::env;
use crate::parser::Parser;

/*
    GRAMMER:
    E -> T {+|- T}
    T -> F {*|/ F}
    F -> Integer | (E) | -F
*/
fn main() {
    let args: &Vec<String> = &env::args().collect();
    let s = args[1].as_str();

    print!("Parsing {}", s);

    let mut p = Parser::new(s);

    match p.parse() {
        Ok(tree) => {
            println!(" => {} = {}", tree.print(), tree.eval());
        },
        Err(e) => println!(" Parsing error: {:?}", e),
    }




}

mod parser {
    use crate::Node::{Add, TreeNode, Integer, Subtract, Negate, Mult, Div, Factorial};

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum Operator {
        Plus,
        Minus,
        Multiplication,
        Division,
        Factorial
    }
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum Token {
        Integer(usize),
        Operator(Operator),
        Negate,
        LeftParenthesis,
        RightParenthesis
    }

    #[derive(Debug)]
    pub enum ParseError {
        SymbolNotAllowed(char),
        NoClosingParenthesis,
        Unknown
    }

    pub struct Parser {
        chars: Vec<char>,
        cursor: usize,
        next_token: Option<Token>
    }

    impl Parser {
        pub fn new(s: &str) -> Parser {
            Parser {
                chars: s.chars().collect(),
                cursor: 0,
                next_token: None
            }
        }
        pub fn cursor(&self) -> usize {
            self.cursor
        }

        pub fn parse(&mut self) -> Result<Box<dyn TreeNode>, ParseError> {
            self.scan_token();
            self.parse_expression()
        }

        pub fn scan_token(&mut self) -> Option<Token> {
            let mut num: Option<usize> = None;

            /* println!("Num is {:?}", num); */
            loop {
                match self.chars.get(self.cursor) {
                    Some(char) => {
                       /*  println!("Cursor: {}, char: {}", self.cursor, char); */

                        match char {
                            char if self.char_is_int(*char) => {
                                self.cursor += 1;

                                if num.is_none() {
                                    num = Some(char.to_digit(10).unwrap() as usize);
                                } else {
                                    num = Some(
                                        num.unwrap() * 10 + char.to_digit(10).unwrap() as usize,
                                    )
                                }
                            }
                            '+' => return self.handle_symbol(num, Token::Operator(Operator::Plus)),
                            '-' => return self.handle_symbol(num, Token::Operator(Operator::Minus)),
                            '*' => return self.handle_symbol(num, Token::Operator(Operator::Multiplication)),
                            '/' => return self.handle_symbol(num, Token::Operator(Operator::Division)),
                            '(' => return self.handle_symbol(num, Token::LeftParenthesis),
                            ')' => return self.handle_symbol(num, Token::RightParenthesis),
                            '!' => return self.handle_symbol(num, Token::Operator(Operator::Factorial)),
                            _ => panic!("Char not allowed ('{}')", char),
                        }
                    }
                    None => {
                        if num.is_some() {
                            let token = Some(Token::Integer(num.unwrap()));
                            self.next_token = token;
                            return token;
                        } else {
                            return None;
                        }
                    },
                }
            }
        }

        fn handle_symbol(&mut self, num: Option<usize>, token: Token) -> Option<Token> {
            if num.is_none() {
                self.cursor += 1;
                let token = Some(token);
                self.next_token = token;
                return token;
            } else {
                let token = Some(Token::Integer(num.unwrap()));
                self.next_token = token;
                return token;
            }
        }
        fn parse_term(&mut self) -> Result<Box<dyn TreeNode>, ParseError> {
            
            let mut a = self.parse_factor()?;

            loop {
                if let Some(next_token) = self.next_token {
                    if next_token == Token::Operator(Operator::Multiplication) {
                        self.scan_token();

                        let b = self.parse_factor()?;
                        a = Box::new(Mult {left: a, right: b});
                    } else if next_token == Token::Operator(Operator::Division) {
                        self.scan_token();

                        let b = self.parse_factor()?;
                        a = Box::new(Div {left: a, right: b});
                    } else {
                        return Ok(a)
                    }
                } else {
                    panic!("No next symbol")
                }   
            }
        }

        fn parse_factor(&mut self) -> Result<Box<dyn TreeNode>, ParseError> {

            match self.next_token.unwrap() {
                Token::Integer(val) => {
                    self.scan_token();
                    Ok(Box::new(Integer{val: val as isize}))
                },
                Token::LeftParenthesis => {
                    self.scan_token();
                    let a = self.parse_expression()?;
                    if self.next_token.unwrap() == Token::RightParenthesis {
                        self.scan_token();
                        return Ok(a)
                    } else {
                        return Err(ParseError::NoClosingParenthesis)
                    }
                },
                Token::Operator(Operator::Minus) => {
                    self.scan_token();
                    return Ok(Box::new(Negate {arg: self.parse_factor()?}));
                }
                _ => Err(ParseError::Unknown)
            }
        }

        fn parse_expression(&mut self) -> Result<Box<dyn TreeNode>, ParseError> {

            let mut a: Box<dyn TreeNode> = self.parse_term()?;

            loop {
                if let Some(next_token) = self.next_token {
                    if next_token == Token::Operator(Operator::Plus) {
                        self.scan_token();

                        let b = self.parse_term()?;
                        a = Box::new(Add {left: a, right: b});
                            
                            
                    } else if next_token == Token::Operator(Operator::Minus) {
                        self.scan_token();

                        let b = self.parse_term()?;
                        a = Box::new(Subtract {left: a, right: b});
                        

                    } else {
                        // More checks needed to detect and prevent "((3+2)" - no closing parenthesis
                        return Ok(a)
                    }
                } else {
                    panic!("No next symbol")
                }   
            }
        }

        fn char_is_int(&self, c: char) -> bool {
            c >= '0' && c <= '9'
        }
    }
}



