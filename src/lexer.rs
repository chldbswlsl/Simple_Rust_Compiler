// ======================= 안승우 파트 =======================


#[derive(Debug, PartialEq, Clone)] //디버그, 비교, 복사 가능
pub enum Token { //Token 열거형
    Number(i32), //숫자
    Ident(String), //단어, 키워드
    Let, If, Else, While,
    Plus, Minus, Assign,
    Equal, Less, Greater,
    Lparen, Rparen, LBrace, RBrace, Semicolon,
    EOF,
}

pub struct Lexer { //Lexer 구조체
    input: Vec<char>, //입력한 문자열을 백터로 저장
    pos: usize, //현재 읽고있는 위치
}

impl Lexer { // Lexer 구조체 구현
    pub fn new(input: &str) -> Self { //새로운 Lexer 생성(생성자)
        Self {
            input: input.chars().collect(), //받은 문자열을 백터로 다 쪼갬
            pos: 0, //위치는 처음으로 초기화
        }
    }

    pub fn next_token(&mut self) -> Token { // 다음 토큰 반환
        while let Some(&c) = self.input.get(self.pos) { //현재 위치에 있는 글자 가져옴
            match c {
                ' ' | '\n' | '\t' => self.pos += 1, //공백이나 줄바꿈, 탭이면 그냥 다음 위치로 이동
                '0'..='9' => return self.lex_number(), //숫자면 lex_number 함수 호출하여 숫자 토큰 생성
                'a'..='z' => return self.lex_ident(), //알파벳이면 lex_ident 함수 호출하여 단어 토큰 생성
                '+' => {self.pos += 1; return Token::Plus; }, // +, - 는 그냥 바로 토큰 생성
                '-' => {self.pos += 1; return Token::Minus; }, 
                '=' => { // = 는 다음 글자가 =인지 확인하여 토큰 생성
                    if self.peek_char() == Some('=') { // == 면 Equal 토큰
                        self.pos += 2;
                        return Token::Equal;
                    } else { // = 면 Assign 토큰
                        self.pos += 1;
                        return Token::Assign;
                    }
                }

                // 나머지 기호들도 바로 토큰 생성
                '<' => {self.pos += 1; return Token::Less;},
                '>' => {self.pos += 1; return Token::Greater;},
                '(' => {self.pos += 1; return Token::Lparen;},
                ')' => {self.pos += 1; return Token::Rparen;},
                '{' => {self.pos += 1; return Token::LBrace;},
                '}' => {self.pos += 1; return Token::RBrace;},
                ';' => {self.pos += 1; return Token::Semicolon;},
                _ => {self.pos += 1; return Token::EOF;}, //그 외의 글자는 EOF 끝처리

            }
        }
        Token::EOF //입력 끝까지 다 읽으면 EOF 반환
    }

    fn peek_char(&self) -> Option<char> { //다음 글자 확인
        self.input.get(self.pos + 1).cloned() //현재 위치 + 1 위치의 글자 반환
    }

    fn lex_number(&mut self) -> Token {
        let mut num = 0; //숫자 저장할 변수

        while let Some(&c) = self.input.get(self.pos) {
            if let Some(d) = c.to_digit(10) { //10진수로 변환 가능하면
                num = num * 10 + d as i32; //숫자 누적
                self.pos += 1; //다음 위치로 이동
            } else {
                break; //숫자가 아니면 종료
            }
        }
        Token::Number(num) //숫자 토큰 반환
    }

    fn lex_ident(&mut self) -> Token {
        let start = self.pos; //단어 시작 위치 저장

        while let Some(&c) = self.input.get(self.pos) {
            if c.is_alphabetic() {
                self.pos += 1; //알파벳이면 다음 위치로 이동
            } else {
                break; //알파벳이 아니면 종료
            }
        }

        let ident: String = self.input[start..self.pos].iter().collect(); //시작 위치부터 현재 위치까지 단어 추출

        match ident.as_str() { //키워드인지 확인
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            _ => Token::Ident(ident), //키워드 아니면 단어 토큰 반환
        }
    }

}