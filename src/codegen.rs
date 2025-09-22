use crate::parser::{Stmt, Expr};

fn expr_to_string(expr: &Expr) -> String {
    match expr {
        Expr::Number(n) => n.to_string(),
        Expr::Variable(v) => v.clone(),
        Expr::BinaryOp(left, op, right) => {
            format!("{} {} {}", expr_to_string(left), op, expr_to_string(right))
        }
    }
}

pub fn generate_code(stmts: &[Stmt]) -> String {
    let mut out = String::new();

    for stmt in stmts {
        match stmt {
            // Let 문 처리: 연산 포함 대입 지원
            Stmt::Let(name, expr) => {
                match expr {
                    Expr::BinaryOp(_, _, _) => {
                        out.push_str(&format!("STORE {} {}\n", name, expr_to_string(expr)));
                    }
                    _ => {
                        out.push_str(&format!("STORE {} {}\n", name, expr_to_string(expr)));
                    }
                }
            }

            // 단순 ExprStmt 처리
            Stmt::ExprStmt(expr) => {
                out.push_str(&format!("PUSH {}\n", expr_to_string(expr)));
            }

            // IF 문 처리
            Stmt::If(cond, then_block, else_block) => {
                out.push_str(&format!("IF {} THEN\n", expr_to_string(cond)));
                out.push_str(&generate_code(then_block));
                if !else_block.is_empty() {
                    out.push_str("ELSE\n");
                    out.push_str(&generate_code(else_block));
                }
                out.push_str("END_IF\n");
            }

            // WHILE 문 처리
            Stmt::While(cond, body) => {
                out.push_str(&format!("WHILE {} DO\n", expr_to_string(cond)));
                out.push_str(&generate_code(body));
                out.push_str("END_WHILE\n");
            }
        }
    }

    out
}
