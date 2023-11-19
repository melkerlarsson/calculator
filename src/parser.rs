use crate::lexer::{Lexer, Token};
use crate::node::{self, TreeNode};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    ChainedOperators,
    IllegalCharacter,
    ExpectedClosingParenthesis,
    ExpectedExpression,
}

type ParseResult = Result<Box<dyn TreeNode>, ParseError>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl Parser<'_> {
    pub fn new(str: &str) -> Parser {
        let lexer = Lexer::new(str);
        Parser {
            lexer,
            current_token: Token::EOF,
        }
    }

    fn read_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> ParseResult {
        self.read_token();
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> ParseResult {
        self.parse_addition()
    }
    fn parse_addition(&mut self) -> ParseResult {
        let mut a = self.parse_multiplication()?;

        loop {
            match self.current_token {
                Token::Plus => {
                    self.read_token();

                    let b = self.parse_multiplication()?;
                    a = Box::new(node::Add { left: a, right: b })
                }
                Token::Minus => {
                    self.read_token();

                    let b = self.parse_multiplication()?;
                    a = Box::new(node::Subtract { left: a, right: b });
                }
                _ => {
                    return Ok(a);
                }
            }
        }
    }
    fn parse_multiplication(&mut self) -> ParseResult {
        let mut a = self.parse_exponentiation()?;

        loop {
            match self.current_token {
                Token::Asterisk => {
                    self.read_token();

                    let b = self.parse_exponentiation()?;
                    a = Box::new(node::Mult { left: a, right: b });
                }
                Token::Slash => {
                    self.read_token();

                    let b = self.parse_exponentiation()?;
                    a = Box::new(node::Div { left: a, right: b });
                }
                _ => {
                    return Ok(a);
                }
            }
        }
    }

    fn parse_exponentiation(&mut self) -> ParseResult {
        let a = self.parse_factorial()?;

        match self.current_token {
            Token::Caret => {
                self.read_token();

                let b = self.parse_exponentiation()?;
                return Ok(Box::new(node::Pow { left: a, right: b }));
            }
            _ => {
                return Ok(a);
            }
        }
    }
    fn parse_factorial(&mut self) -> ParseResult {
        let a = self.parse_basic()?;

        match self.current_token {
            Token::ExclamationMark => {
                self.read_token();
                return Ok(Box::new(node::Factorial { arg: a }));
            }
            _ => {
                return Ok(a);
            }
        }
    }
    fn parse_basic(&mut self) -> ParseResult {
        match self.current_token {
            Token::Constant(constant) => {
                self.read_token();
                return Ok(Box::new(node::Constant { symbol: constant }));
            }
            Token::Integer(val) => {
                self.read_token();
                return Ok(Box::new(node::Integer { val: val as isize }));
            }
            Token::Float(val) => {
                self.read_token();
                return Ok(Box::new(node::Float { val: val.into() }))
            }
            Token::LeftParenthesis => {
                self.read_token();
                let a: Box<dyn TreeNode> = self.parse_expression()?;

                match self.current_token {
                    Token::RightParenthesis => {
                        self.read_token();
                        return Ok(a);
                    }
                    _ => {
                        return Err(ParseError::ExpectedClosingParenthesis);
                    }
                }
            }
            Token::Function(function) => {
                self.read_token();
                let a: Box<dyn TreeNode> = self.parse_basic()?;
            
                return Ok(Box::new(node::Function { arg: a, function }));
            }
            Token::Illegal(_) => {
                return Err(ParseError::IllegalCharacter);
            }
            Token::EOF => return Err(ParseError::ExpectedExpression),
            Token::Minus => {
                return Ok(Box::new(node::Integer { val: 0 } ))
            }
            _ => return Err(ParseError::ExpectedExpression),
        }
    }
}

mod tests {
    use super::*;

    fn test(input: &str, expected: f64) {
        let mut parser = Parser::new(input);
        let res = parser.parse();
        assert_eq!(res.unwrap().eval(), expected);
    }

    #[test]
    fn addition() {
        test("2 + 3 +1 +4", 10.0);
    }

    #[test]
    fn subtraction() {
        test("4-2", 2.0);
    }

    #[test]
    fn multiplication_positive_positive() {
        test("2 * 6", 12.0);
    }

    #[test]
    fn multiplication_negative_positive() {
        test("(-2) * 6", -12.0);
    }

    #[test]
    fn negative_number() {
        test("(-2)", -2.0);
    }

    #[test]
    fn power() {
        test("2^3", 8.0);
    }

    #[test]
    fn factorial() {
        test("4!", 24.0);
        test("6!", 720.0)
    }

    #[test]
    fn order_of_operation() {
        test("2*(10-2^3+1)!", 12.0)
    }

    #[test]
    fn chained_operators() {
        let mut parser = Parser::new("1+*2");
        let result = parser.parse();

        assert_eq!(result.err().unwrap(), ParseError::ChainedOperators);
    }

    #[test]
    fn constant() {
        test("g", 9.82);
    }
}
