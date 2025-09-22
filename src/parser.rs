use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Variable(String),
    BinaryOp(Box<Expr>, String, Box<Expr>),
}

#[derive(Debug)]
pub enum Stmt {
    Let(String, Expr),
    ExprStmt(Expr),
    If(Expr, Vec<Stmt>, Vec<Stmt>),
    While(Expr, Vec<Stmt>),
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self { Self { tokens, pos: 0 } }

    fn current(&self) -> &Token { self.tokens.get(self.pos).unwrap_or(&Token::EOF) }
    fn next(&mut self) { self.pos +=1; }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = vec![];
        while !matches!(self.current(), Token::EOF) {
            stmts.push(self.parse_stmt());
        }
        stmts
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.current() {
            Token::Let => {
                self.next();
                if let Token::Ident(name) = self.current() {
                    let name = name.clone();
                    self.next();
                    if let Token::Assign = self.current() { self.next(); }
                    let expr = self.parse_expr();
                    if let Token::Semicolon = self.current() { self.next(); }
                    Stmt::Let(name, expr)
                } else { panic!("Expected identifier"); }
            }
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            _ => {
                let expr = self.parse_expr();
                if let Token::Semicolon = self.current() { self.next(); }
                Stmt::ExprStmt(expr)
            }
        }
    }

    fn parse_expr(&mut self) -> Expr {
        let mut left = match self.current() {
            Token::Number(n) => { let val = *n; self.next(); Expr::Number(val) },
            Token::Ident(name) => { let var = name.clone(); self.next(); Expr::Variable(var) },
            _ => { self.next(); Expr::Number(0) }
        };

        while matches!(self.current(), Token::Plus | Token::Minus | Token::Equal | Token::Less | Token::Greater) {
            let op = match self.current() {
                Token::Plus => "+",
                Token::Minus => "-",
                Token::Equal => "==",
                Token::Less => "<",
                Token::Greater => ">",
                _ => "",
            }.to_string();
            self.next();
            let right = self.parse_expr();
            left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
        }

        left
    }

    fn parse_if(&mut self) -> Stmt {
        self.next(); // skip if
        let cond = self.parse_expr();
        let then_block = self.parse_block();
        let mut else_block = vec![];
        if let Token::Else = self.current() { self.next(); else_block = self.parse_block(); }
        Stmt::If(cond, then_block, else_block)
    }

    fn parse_while(&mut self) -> Stmt {
        self.next(); // skip while
        let cond = self.parse_expr();
        let body = self.parse_block();
        Stmt::While(cond, body)
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        let mut stmts = vec![];
        if let Token::LBrace = self.current() { self.next(); } else { return stmts; }
        while !matches!(self.current(), Token::RBrace | Token::EOF) {
            stmts.push(self.parse_stmt());
        }
        if let Token::RBrace = self.current() { self.next(); }
        stmts
    }
}
