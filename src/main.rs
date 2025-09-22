mod lexer;
mod parser;
mod codegen;

use lexer::Lexer;
use parser::Parser;
use codegen::generate_code;
use std::fs;

fn main() {
    let code = fs::read_to_string("example/example.rs")
        .expect("Failed to read file");

    // Lexer
    let mut lexer = Lexer::new(&code);
    let mut tokens = vec![];
    loop {
        let tok = lexer.next_token();
        if tok == lexer::Token::EOF { break; }
        tokens.push(tok);
    }

    // Parser
    let mut parser = Parser::new(&tokens);
    let ast = parser.parse();

    // Codegen
    let machine_code = generate_code(&ast);

    println!("{}", machine_code);

    // 파일 저장
    std::fs::write("output.mc", &machine_code).expect("Failed to write machine code");
    println!("Machine code saved to output.mc");
}
