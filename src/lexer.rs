use std::{collections::VecDeque, str::Chars};

use phf::phf_map;

static CONSTANTS: phf::Map<&'static str, Constant> = phf_map! {
    "g" => Constant::g,
    "pi" => Constant::pi,
    "e" => Constant::e
};

static FUNCTIONS: phf::Map<&'static str, Function> = phf_map! {
    "sin" =>  Function::sin,
    "ln" => Function::ln,
};

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Constant {
    g,
    pi,
    e
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Function {
    sin,
    ln,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    EOF,
    Illegal(String),
    Integer(u32),
    Float(f32),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParenthesis,
    RightParenthesis,
    ExclamationMark, // "!"
    Caret,           // "^"
    Constant(Constant),
    Function(Function),
}

pub struct Lexer<'a> {
    input: Chars<'a>,
    current: char,
    peek_buffer: VecDeque<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: input.chars(),
            current: '\0',
            peek_buffer: VecDeque::new(),
        };

        lexer.read_char();
        lexer
    }

    fn peek(&mut self) -> char {
        if let Some(c) = self.peek_buffer.front() {
            *c
        } else {
            let next = self.input.next().unwrap_or('\0');
            self.peek_buffer.push_back(next);
            next
        }
    }

    fn read_char(&mut self) {
        if let Some(c) = self.peek_buffer.pop_front() {
            self.current = c;
            return;
        }
        self.current = self.input.next().unwrap_or('\0');
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.current {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '!' => Token::ExclamationMark,
            '^' => Token::Caret,
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            '\0' => Token::EOF,
            '.' => {
                self.read_char();
                if self.current.is_digit(10) {
                    let mut num = self.current.to_digit(10).unwrap();
                    self.read_char();

                    while self.current.is_digit(10) {
                        num = num * 10 + self.current.to_digit(10).unwrap();
                        self.read_char();
                    }

                    return Token::Float((".".to_owned() + &num.to_string()).parse::<f32>().unwrap());
                } else {
                    return Token::Illegal(".".to_string());
                }
            }
            c => {
                if c.is_digit(10) {
                    return self.read_number();
                } else if c.is_alphabetic() {
                    return self.read_string();
                } else {
                    Token::Illegal(c.to_string())
                }
            }
        };

        self.read_char();
        token
    }

    fn read_number(&mut self) -> Token {
        let mut num = self.current.to_digit(10).unwrap();
        self.read_char();

        while self.current.is_digit(10) {
            num = num * 10 + self.current.to_digit(10).unwrap();
            self.read_char();
        }

        if self.current == '.' {
            self.read_char();

            if self.current.is_digit(10) {
                let mut num2: u32 = self.current.to_digit(10).unwrap();
                self.read_char();

                while self.current.is_digit(10) {
                    num2 = num2 * 10 + self.current.to_digit(10).unwrap();
                    self.read_char();
                }

                Token::Float(
                    (num.to_string() + "." + &num2.to_string())
                        .parse::<f32>()
                        .unwrap(),
                )
            } else {
                Token::Illegal(".".to_string())
            }
        } else {
            Token::Integer(num)
        }
    }

    fn read_string(&mut self) -> Token {
        let mut str = String::from(self.current);
        self.read_char();

        while self.current.is_alphabetic() {
            str.push(self.current);
            self.read_char();
        }

        if let Some(constant) = CONSTANTS.get(str.as_str()) {
            Token::Constant(*constant)
        } else if let Some(function) = FUNCTIONS.get(str.as_str()) {
            Token::Function(*function)
        } else {
            Token::Illegal(str)
        }
    }

    fn skip_whitespace(&mut self) {
        while self.current.is_whitespace() {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic_tokens() {
        let mut lexer = Lexer::new("1 # + 2 * (4-2^3)!");
        let expected = [
            Token::Integer(1),
            Token::Illegal('#'.to_string()),
            Token::Plus,
            Token::Integer(2),
            Token::Asterisk,
            Token::LeftParenthesis,
            Token::Integer(4),
            Token::Minus,
            Token::Integer(2),
            Token::Caret,
            Token::Integer(3),
            Token::RightParenthesis,
            Token::ExclamationMark,
            Token::EOF,
        ];

        for token in expected {
            assert_eq!(token, lexer.next_token())
        }
    }
    #[test]
    fn no_input() {
        let mut lexer = Lexer::new("");
        let expected = [Token::EOF, Token::EOF];

        for token in expected {
            assert_eq!(token, lexer.next_token());
        }
    }

    #[test]
    fn integer() {
        let mut lexer = Lexer::new("4845 12");
        let expected = [Token::Integer(4845), Token::Integer(12), Token::EOF];

        for token in expected {
            assert_eq!(token, lexer.next_token());
        }
    }

    #[test]
    fn string() {
        let mut lexer = Lexer::new("g pi Me");
        let expected = [
            Token::Constant(Constant::g),
            Token::Constant(Constant::pi),
            Token::Illegal("Me".to_string()),
        ];

        for token in expected {
            assert_eq!(token, lexer.next_token());
        }
    }

    #[test]
    fn functions() {
        let mut lexer = Lexer::new("sin(2) - ln(4)");
        let expected = [
            Token::Function(Function::sin),
            Token::LeftParenthesis,
            Token::Integer(2),
            Token::RightParenthesis,
            Token::Minus,
            Token::Function(Function::ln),
            Token::LeftParenthesis,
            Token::Integer(4),
            Token::RightParenthesis,
        ];

        for token in expected {
            assert_eq!(token, lexer.next_token());
        }
    }

    #[test]
    fn float() {
        let mut lexer = Lexer::new("1.9 234.1 2.234 123.456 0.9384 .343");
        let expected = [
            Token::Float(1.9),
            Token::Float(234.1),
            Token::Float(2.234),
            Token::Float(123.456),
            Token::Float(0.9384),
            Token::Float(0.343),
        ];

        for token in expected {
            assert_eq!(token, lexer.next_token())
        }

    }

    #[test]
    fn peek() {
        let mut lexer = Lexer::new("12 g");

        assert_eq!(Token::Integer(12), lexer.next_token());
        assert_eq!('g', lexer.peek());
        assert_eq!(
            Token::Constant(Constant::g),
            lexer.next_token()
        );
    }
}
