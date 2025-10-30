mod lexer;  // lexer.rs 모듈을 가져옴 (토큰화 기능)
mod parser; // parser.rs 모듈을 가져옴 (토큰을 AST로 변환)
mod codegen; // codegen.rs 모듈을 가져옴 (AST를 가상 머신 코드로 변환)

use lexer::{Lexer, Token}; // Lexer 구조체와 Token 열거형을 사용
use parser::{Parser, Stmt, Expr}; // Parser 구조체, AST 문장/표현식 타입 사용
use codegen::CodeGen; // CodeGen 구조체 사용

fn main() {
    // 테스트용 소스 코드 문자열
    let src = "
        let x = 10 + 20;
        if x > 15 {
            x = x - 5;
        } else {
            x = x + 5;
        }
        while x < 30 {
            x = x + 1;
        }
    ";

    // 1. Lexer: 문자열을 토큰으로 변환
    let mut lexer = Lexer::new(src); // Lexer 객체 생성, 입력 문자열을 초기화
    let mut tokens = vec![]; // 토큰들을 저장할 벡터 생성
    loop {
        let tok = lexer.next_token(); // Lexer에서 다음 토큰 가져오기
        if tok == Token::EOF { break; } // EOF 토큰이면 루프 종료
        tokens.push(tok); // 벡터에 토큰 추가
    }

    // 2. Parser: 토큰을 AST로 변환
    let mut parser = Parser::new(&tokens); // Parser 객체 생성, 토큰 슬라이스 전달
    let stmts: Vec<Stmt> = parser.parse(); // parse() 호출하여 전체 토큰을 AST 문장 벡터로 변환

    // 3. CodeGen: AST를 가상 머신 코드로 변환
    let mut codegen = CodeGen::new(); // CodeGen 객체 생성
    let code = codegen.gen_program(&stmts); // AST 문장을 가상 머신 명령어 문자열로 변환

    // 결과 출력
    println!("Generated code:\n{}", code); // 생성된 가상 머신 코드를 화면에 출력
}
