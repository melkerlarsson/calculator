#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Constant {
    pub val: f64,
    pub symbol: char,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    EOF,
    Illegal(char),
    Integer(usize),
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
    ch: char,
}

impl Lexer {
    pub fn new(s: &str) -> Lexer {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        let first = chars.get(0).unwrap();

        Lexer {
            chars: chars.clone(),
            cursor: 0,
            length: len,
            ch: *first,
        }
    }

    pub fn read_char(&mut self) {
        if self.cursor >= self.length {
            self.ch = '\0';
        }
    }

    pub fn next_token(&mut self) -> Token {

        let token = match self.ch {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiplication,
            '/' => Token::Division,
            '!' => Token::ExclamationMark,
            '^' => Token::Caret,
            '(' => Token::LeftParenthesis,
            '\0' => Token::EOF,
            _ => Token::Illegal(self.ch)

        };

        token
    }

    fn should_skip(&self, char: &char) -> bool {
        char == &' ' 
    }

    fn char_is_int(&self, c: char) -> bool {
        c >= '0' && c <= '9'
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
}

