#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i32),
    Ident(String),
    Let, If, Else, While,
    Plus, Minus, Assign,
    Equal, Less, Greater,
    LParen, RParen, LBrace, RBrace, Semicolon,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self { input: input.chars().collect(), pos: 0 }
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(&c) = self.input.get(self.pos) {
            match c {
                ' ' | '\n' | '\t' => self.pos += 1,
                '0'..='9' => return self.lex_number(),
                'a'..='z' => return self.lex_ident(),
                '+' => { self.pos += 1; return Token::Plus; },
                '-' => { self.pos += 1; return Token::Minus; },
                '=' => {
                    if self.peek_char() == Some('=') {
                        self.pos += 2;
                        return Token::Equal;
                    } else { self.pos += 1; return Token::Assign; }
                }
                '<' => { self.pos += 1; return Token::Less; }
                '>' => { self.pos += 1; return Token::Greater; }
                '(' => { self.pos += 1; return Token::LParen; },
                ')' => { self.pos += 1; return Token::RParen; },
                '{' => { self.pos += 1; return Token::LBrace; },
                '}' => { self.pos += 1; return Token::RBrace; },
                ';' => { self.pos += 1; return Token::Semicolon; },
                _ => { self.pos += 1; return Token::EOF; }
            }
        }
        Token::EOF
    }

    fn peek_char(&self) -> Option<char> { self.input.get(self.pos+1).copied() }

    fn lex_number(&mut self) -> Token {
        let mut num = 0;
        while let Some(&c) = self.input.get(self.pos) {
            if let Some(d) = c.to_digit(10) {
                num = num * 10 + d as i32;
                self.pos +=1;
            } else { break; }
        }
        Token::Number(num)
    }

    fn lex_ident(&mut self) -> Token {
        let start = self.pos;
        while let Some(&c) = self.input.get(self.pos) {
            if c.is_alphabetic() { self.pos +=1; } else { break; }
        }
        let ident: String = self.input[start..self.pos].iter().collect();
        match ident.as_str() {
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            _ => Token::Ident(ident),
        }
    }
}
