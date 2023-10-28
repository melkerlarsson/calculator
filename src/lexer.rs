#[derive(Clone, Debug, PartialEq)]
pub struct Constant {
    pub val: f64,
    pub symbol: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    EOF,
    Illegal(char),
    Integer(u32),
    Plus,
    Minus,
    Multiplication,
    Division,
    LeftParenthesis,
    RightParenthesis,
    ExclamationMark, // "!"
    Caret, // "^"
    Constant(Constant),
}

pub struct Lexer {
    chars: Vec<char>,
    length: usize,
    cursor: usize,
    current: char,
}

impl Lexer {
    pub fn new(s: &str) -> Lexer {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();

        let mut lexer = Lexer {
            chars: chars.clone(),
            cursor: 0,
            length: len,
            current: '\0',
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        self.current = *self.chars.get(self.cursor).unwrap_or(&'\0');
        self.cursor += 1;
    }

    fn read_ident(&mut self) -> String {
        let mut ident = String::new();
        while self.current.is_alphabetic() || self.current == '_' || self.current.is_ascii_digit() {
            ident.push(self.current);
            self.read_char();
        }
        ident
    }

    pub fn next_token(&mut self) -> Token {

        self.skip_whitespace();

        let token = match self.current {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiplication,
            '/' => Token::Division,
            '!' => Token::ExclamationMark,
            '^' => Token::Caret,
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            '\0' => Token::EOF,
            c => {
                if c.is_digit(10) {
                    return self.read_integer();
                } else if c.is_alphabetic() {
                    return self.read_string();
                } else {
                    Token::Illegal(c)
                }
            }

        };

        self.read_char();
        token
    }

    fn read_integer(&mut self) -> Token {
        let mut num = self.current.to_digit(10).unwrap();
        self.read_char();

        while self.current.is_digit(10) {
            num = num * 10 + self.current.to_digit(10).unwrap();
            self.read_char();
        }

        Token::Integer(num)
    }

    fn read_string(&mut self) -> Token {
        let mut str = String::from(self.current);
        self.read_char();

        while self.current.is_alphabetic() {
            str.push(self.current);
            self.read_char();
        }

        Token::Constant(Constant { val: 0.0, symbol: str })
        
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
            Token::Illegal('#'),
            Token::Plus,
            Token::Integer(2),
            Token::Multiplication,
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
        let mut lexer = Lexer::new("G Me sin");
        let expected = [
            Token::Constant(Constant { val: 0.0, symbol: String::from("G")}),
            Token::Constant(Constant { val: 0.0, symbol: String::from("Me")}),
            Token::Constant(Constant { val: 0.0, symbol: String::from("sin")})
        ];

        for token in expected {
            assert_eq!(token, lexer.next_token());
        }
    }

    
}

