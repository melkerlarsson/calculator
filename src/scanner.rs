use phf::{phf_map};

static CONSTANTS: phf::Map<char, f64> = phf_map! {
    'g' => 9.82,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Constant {
    pub val: f64,
    pub symbol: char,

}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Integer(usize),
    Plus,
    Minus,
    Multiplication,
    Division,
    LeftParenthesis,
    RightParenthesis,
    Factorial,
    Power,
    Constant(Constant)
}

#[derive(Debug)]
pub enum ScanError {
    SymbolNotAllowed(char),
}

pub struct Scanner {
    chars: Vec<char>,
    cursor: usize,
    pub next_token: Option<Token>,
}

impl Scanner {
    pub fn new(s: &str) -> Scanner {
        Scanner {
            chars: s.chars().collect(),
            cursor: 0,
            next_token: None,
        }
    }

    pub fn scan_token(&mut self) -> Result<Option<Token>, ScanError> {
        let mut num: Option<usize> = None;
        let mut str: Option<String> = None;

        /* println!("Num is {:?}", num); */
        loop {
            match self.chars.get(self.cursor) {
                Some(char) => {
                    /*  println!("Cursor: {}, char: {}", self.cursor, char); */

                    match char {

                        char if self.should_skip(char) => {
                            self.cursor += 1;
                        }

                        char if char.is_ascii_alphabetic() => {
                            self.cursor += 1;
                            if let Some(val) = CONSTANTS.get(char) {
                                let token = Some(Token::Constant(Constant { val: *val, symbol: *char }));
                                self.next_token = token;
                                return Ok(token);
                            }  else {
                                return Err(ScanError::SymbolNotAllowed(*char)); 
                            }

                            
                            //str = Some(format!("{}{}", str.unwrap_or(String::from("")), char));
                        }

                        char if char.is_ascii_digit() => {
                            self.cursor += 1;
                            num = Some(num.unwrap_or(0) * 10 + char.to_digit(10).unwrap() as usize);
                        }
                        '+' => return Ok(self.handle_symbol(num, Token::Plus)),
                        '-' => return Ok(self.handle_symbol(num, Token::Minus)),
                        '*' => return Ok(self.handle_symbol(num, Token::Multiplication)),
                        '/' => return Ok(self.handle_symbol(num, Token::Division)),
                        '(' => return Ok(self.handle_symbol(num, Token::LeftParenthesis)),
                        ')' => return Ok(self.handle_symbol(num, Token::RightParenthesis)),
                        '!' => return Ok(self.handle_symbol(num, Token::Factorial)),
                        '^' => return Ok(self.handle_symbol(num, Token::Power)),
                        _ => return Err(ScanError::SymbolNotAllowed(*char)),
                    }
                }
                None => {
                    if let Some(num) = num {
                        let token = Some(Token::Integer(num));
                        self.next_token = token;
                        return Ok(token);
                    } else {
                        return Ok(None);
                    }
                }
            }
        }
    }

    fn should_skip(&self, char: &char) -> bool {
        char == &' ' 
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

    fn char_is_int(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }
}
