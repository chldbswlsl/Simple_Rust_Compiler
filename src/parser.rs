// ======================= 안승우 파트 =======================
// 최종본

use crate::lexer::Token;
#[derive(Debug)]

// AST 표현식 정의
pub enum Expr {
    Number(i32),                                //숫자
    Variable(String),                           //변수
    BinaryOp(Box<Expr>, String, Box<Expr>),     //이항 연산 (left, op, right)
}

#[derive(Debug)]

// AST 문장(statement) 정의
pub enum Stmt {
    Let(String, Expr),                  // let 변수 선언
    ExprStmt(Expr),                     // 일반 표현식 문장
    If(Expr, Vec<Stmt>, Vec<Stmt>),     // if 문 (조건, then 블록, else 블록)
    While(Expr, Vec<Stmt>),             // while 문 (조건, 블록)
}

// 파서 구조체
pub struct Parser<'a> {
    tokens: &'a [Token],    //파싱할 토큰 배열 (참조)
    pos: usize,             //현재 위치
}

impl<'a> Parser<'a> {
    // 새로운 파서 생성
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }      //pos = 0부터 시작
    }

    //현재 토큰 가져오기
    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)    // 범위 벗어나면 EOF 반환
    }

    // 다음 토큰으로 이동
    fn next(&mut self) {
        self.pos += 1;
    }

    // 전체 프로그램 파싱 (문장 단위)
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = vec![];
        while !matches!(self.current(), Token::EOF) {   // EOF 전까지
            stmts.push(self.parse_stmt()); // 한 문장씩 파싱
        }
        stmts
    }

    // 한 문장(statement) 파싱
    fn parse_stmt(&mut self) -> Stmt {
        match self.current() {
            Token::Let => {                                                 //let 문
                self.next();                                                // 'let' 토큰 소비
                if let Token::Ident(name) = self.current() {                //변수 이름 확인
                    let name = name.clone();                                // 문자열 소유권 확보
                    self.next();                                            // 식별자 토큰 소비
                    if let Token::Assign = self.current() {self.next();}    //'=' 있으면 소비
                    let expr = self.parse_expr();                           //오른쪽 식 파싱
                    if let Token::Semicolon = self.current() {              //';' 있으면 소비
                        self.next();                                        
                    }
                    Stmt::Let(name, expr)                                   //AST 변환
                } else {
                    panic!("Expected Ident");                               // 식별자 없으면 에러
                }
            }
            Token::If => self.parse_if(),                                   // if 문 파싱
            Token::While => self.parse_while(),                             // while 문 파싱
            _ => {                                                          // 나머지 -> 표현식 문장
                let expr = self.parse_expr();                               // 식 파싱
                if let Token::Semicolon = self.current() {                  //';' 있으면 소비
                    self.next();
                }
                Stmt::ExprStmt(expr)                                        // AST 반환
            }
        }
    }

    // 한 표현식 파싱
    fn parse_expr(&mut self) -> Expr {
        // 좌측 표현식 초기화
        let mut left = match self.current() {
            Token::Number(n) => {let val = *n; self.next(); Expr::Number(val)},                 //숫자
            Token::Ident(name) => {let var = name.clone(); self.next(); Expr::Variable(var)}    //변수
            _ => {self.next(); Expr::Number(0)}                                                 // 기타 -> 0으로 처리
        };

        // 연산자 처리 (재귀적 이항 연산)
        while matches!(self.current(), Token::Plus | Token::Minus | Token::Equal | Token::Less | Token::Greater){
            let op = match self.current() {
                Token::Plus => "+",
                Token::Minus => "-",
                Token::Equal => "==",
                Token::Less => "<",
                Token::Greater => ">",
                _ => "",
            }.to_string();
            self.next();                                                                         // 연산자 토큰 소비
            let right = self.parse_expr();                                                            // 오른쪽 표현식 파싱

            left = Expr::BinaryOp(Box::new(left), op, Box::new(right));                          // AST 갱신
        }
        left                                                                                     // 최종 표현식 반환
    }

    // if 문 파싱
    fn parse_if(&mut self) -> Stmt {
        self.next();                                // 'if' 토큰 소비
        let cond = self.parse_expr();               // 조건식 파싱
        let then_block = self.parse_block();        // then 블록 파싱 {}
        let mut else_block = vec![];                
        if let Token::Else = self.current() {       // else 블록이 있으면
            self.next();
            else_block = self.parse_block();        // else 블록 파싱
        }
        Stmt::If(cond, then_block, else_block)      // AST 반환
    }

    // while 문 파싱
    fn parse_while(&mut self) -> Stmt {
        self.next();                            // 'while' 토큰 소비
        let cond = self.parse_expr();           // 조건식 파싱
        let body = self.parse_block();          // 본문 블록 파싱
        Stmt::While(cond, body)                // AST 반환
    }

    // 중괄호 블록 { ... } 파싱
    fn parse_block(&mut self) -> Vec<Stmt> {
        let mut stmts = vec![];
        if let Token::LBrace = self.current() {                                 // '{' 없으면 빈 블록
            self.next();
        } else {
            return stmts;
        }
        while !matches!(self.current(), Token::RBrace | Token::EOF){            // '}'나 EOF 나올 때까지
            stmts.push(self.parse_stmt());                                      // 문장 파싱
        }

        if let Token::RBrace = self.current() {                                 // '}' 소비
            self.next();
        }
        stmts                                                                   // 블록 반환


    }
}