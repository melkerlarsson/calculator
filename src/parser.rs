use crate::node::*;
use crate::scanner::{Scanner, Token, ScanError};

pub struct Parser {
    scanner: Scanner,
}

#[derive(Debug)]
pub enum Error {
    ParseError(ParseError),
    ScanError(ScanError)
}

#[derive(Debug)]
pub enum ParseError {
    ExpectedExpression,
    ChainedOperators,
    ExpectedClosingParenthesis,
}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Error::ParseError(value)
    }
}

impl From<ScanError> for Error {
    fn from(value: ScanError) -> Self {
        Error::ScanError(value)
    }
}

type ParseResult = Result<Box<dyn TreeNode>, Error>;

impl Parser {
    pub fn new(scanner: Scanner) -> Parser {
        Parser { scanner }
    }

    pub fn parse(&mut self) -> ParseResult {
        self.scanner.scan_token()?;
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> ParseResult {
        self.parse_addition()
    }

    fn parse_addition(&mut self) -> ParseResult {
        let mut a = self.parse_multiplication()?;

        loop {
            let next_token = self.scanner.next_token;
            if next_token.is_some() {
                match next_token.unwrap() {
                    Token::Plus => {
                        self.scanner.scan_token()?;

                        let b = self.parse_multiplication()?;
                        a = Box::new(Add { left: a, right: b });
                    }
                    Token::Minus => {
                        self.scanner.scan_token()?;

                        let b = self.parse_multiplication()?;
                        a = Box::new(Subtract { left: a, right: b });
                    }
                    _ => {
                        return Ok(a);
                    }
                }
            } else {
                return Ok(a);
            }
        }
    }

    fn parse_multiplication(&mut self) -> ParseResult {
        let mut a = self.parse_exponentiation()?;

        loop {
            let next_token = self.scanner.next_token;
            if next_token.is_some() {
                match next_token.unwrap() {
                    Token::Multiplication => {
                        self.scanner.scan_token()?;

                        let b = self.parse_exponentiation()?;
                        a = Box::new(Mult { left: a, right: b });
                    }
                    Token::Division => {
                        self.scanner.scan_token()?;

                        let b = self.parse_exponentiation()?;
                        a = Box::new(Div { left: a, right: b });
                    }
                    _ => {
                        return Ok(a);
                    }
                }
            } else {
                return Ok(a);
            }
        }
    }

    fn parse_exponentiation(&mut self) -> ParseResult {
        

        let a = self.parse_factorial()?;
        let next_token = self.scanner.next_token;

        if next_token.is_some() {
            match next_token.unwrap() {
                Token::Power => {
                    self.scanner.scan_token()?;
                    return Ok(Box::new(Pow{left: a, right: self.parse_exponentiation()?}))
                }
                _ => return Ok(a),
            }
        } else {
            return Err(Error::from(ParseError::ExpectedExpression));
        }
    }

    fn parse_factorial(&mut self) -> ParseResult {
        let a = self.parse_basic()?;

        if let Some(next_token) = self.scanner.next_token {
            match next_token {
                Token::Factorial => {
                    self.scanner.scan_token()?;
                    return Ok(Box::new(Factorial { arg: a}));
                },
                _ => {
                    return Ok(a);
                }
            }
        } else {
            return Ok(a);
        }
    }

    fn parse_basic(&mut self) -> ParseResult {
        let next_token = self.scanner.next_token;

        if next_token.is_some() {
            match next_token.unwrap() {
                Token::Constant(constant) => {
                    self.scanner.scan_token()?;
                    return Ok(Box::new(Constant { val: constant.val, symbol: constant.symbol }))
                },  
                Token::Integer(val) => {
                    self.scanner.scan_token()?;
                    return Ok(Box::new(Integer { val: val as isize }));
                },
                Token::LeftParenthesis => {
                    self.scanner.scan_token()?;

                    let a = self.parse_expression()?;

                    if let Some(next_token) = self.scanner.next_token {
                        match next_token {
                            Token::RightParenthesis => {
                                self.scanner.scan_token()?;
                                return Ok(a);
                            },
                            _ => return Err(Error::from(ParseError::ExpectedClosingParenthesis))
                        }
                    } else {
                        return Err(Error::from(ParseError::ExpectedClosingParenthesis));
                    }

                },
                _ => return Err(Error::from(ParseError::ExpectedExpression)),
            }
        } else {
            return Err(Error::from(ParseError::ExpectedExpression));
        }
    }
}



/* 
EXPRESSION
    : ADDITION
    ;

ADDITION
    : MULTIPLICATION {('+' | '-') MULTIPLICATION}
    ;

MULTIPLICATION
    : EXPONENTIATION {('*' | '/') EXPONENTIATION}
    ;

EXPONENTIATION
    : EXPONENTIATION '^' BASIC
    | BASIC
    ;



BASIC
    : number
    | identifier
    | '(' EXPRESSION ')'
    ; 
*/
