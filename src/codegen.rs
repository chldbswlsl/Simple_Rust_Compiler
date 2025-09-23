// ======================= 노희래 파트 =======================
// 최종본
use std::fmt::Write;

pub struct CodeGen {
  code: Vec<String>, // 생성된 코드를 문자열로 저장
  label_cnt: usize   // 레이블 이름에 붙일 번호
}

impl CodeGen {
  /// CodeGen 생성자
  pub fn new() -> Self {
    Self {
      code: Vec::new(), // 빈 코드 벡터
      label_cnt: 0,     // 0부터 시작
    }
  }

  fn emit<S: AsRef<str>>(&mut self, s: S) {
    self.code.push(s.as_ref().to_string()); // 문자를 String으로 변환
    // 문자열(코드)을 CodeGen의 내부 코드 저장소에 넣는 역할
  }

  /// 레이블 이름 만들기
  fn new_label(&mut self, prefix: &str) -> String {
    let l = format!("{}_{}", prefix, self.label_cnt); // prefix와 현재 카운터로 레이블 문자열 생성
    self.label_cnt += 1; // 카운터 증가
    l // 레이블 이름 반환
  }

  /// 프로그램 전체 코드 생성
  pub fn gen_program(&mut self, stmts: &[crate::Stmt]) -> String {
    for stmt in stmts {    // Stmt 순회
      self.gen_stmt(stmt); // 문장별로 코드 생성
    }
    self.emit("HALT");     // 프로그램 종료 명령어 추가
    self.code.join("\n")   // 저장된 명령어를 줄바꿈으로 이어 붙여 반환
  }

  /// 문장(Stmt) 단위 코드 생성
  fn gen_stmt(&mut self, stmt: &crate::Stmt) {
    match stmt {

      // let 변수 선언 (예 let x = 1;)
      crate::Stmt::Let(name, expr) => {
        self.gen_expr(expr);                  // 오른쪽 식을 계산하여 결과를 스택에 남김
        self.emit(format!("STORE {}", name)); // 스택 top 값을 해당 변수 이름으로 메모리에 저장
      }

      // 일반 표현식 문장(예 x + 1;)
      crate::Stmt::ExprStmt(expr) => {
        self.gen_expr(expr); // 식을 평가 후 스택에 결과를 남김
        self.emit("POP");    // 평가 결과가 사용되지 않으므로 스택에서 제거
      }

      // if 문 (조건, then,else)
      crate::Stmt::If(cond, then_block, else_block) => {
        self.gen_expr(cond);                      // 조건식을 평가해서 스택에 0/1(참 거짓) 값을 남김
        let else_label = self.new_label("Lelse"); // else 블록으로 점프할 레이블 생성
        let end_label = self.new_label("Lend");   // if 전체 종료 위치를 가리킬 레이블 생성

        self.emit(format!("JZ {}", else_label));  // 조건이 0(거짓) 이면 else 레이블로 점프

        // then 코드 생성
        for s in then_block {
          self.gen_stmt(s); // then 블록 안의 각 문장에 대해 재귀적 코드 생성
        }
        self.emit(format!("JMP {}", end_label));    // then이 끝난 뒤 else를 건너뛰기 위해 end로 점프

        // else 코드 생성
        self.emit(format!("LABEL {}",else_label)); // else 레이블 위치 정의
        for s in else_block {
          self.gen_stmt(s); // else 블록 안의 문장들 코드 생성
        }

        // if 종료 지점
        self.emit(format!("LABEL {}", end_label)); // if-else가 끝나는 지점에 레이블 정의
      }

      // while 문 (조건, 반복)
      crate::Stmt::While(cond, body) => {
        let start_label = self.new_label("Lstart");  // 루프 시작 레이블 생성
        let end_label = self.new_label("Lend");      // 루프가 끝날 때 사용할 레이블 생성

        self.emit(format!("LABEL {}", start_label)); // 루프 시작 위치 표시
        self.gen_expr(cond);                         // 조건식을 평가 후 스택에 0/1을 남김
        self.emit(format!("JZ {}", end_label));      // 조건이 0(false) 이면 루프 종료 레이블로 이동

        // 루프 본문 생성
        for s in body {
          self.gen_stmt(s); // 루프 바디의 각 문장에 대해 코드 생성
        }

        self.emit(format!("JMP {}", start_label)); // 본문 완료 후 다시 루프 시작으로 점프
        self.emit(format!("LABEL {}", end_label)); // 루프 종료 위치 레이블 정의
      }
    }
  }

  /// 표현식(Expr) 단위 코드 생성
  fn gen_expr(&mut self, expr: &crate::Expr) {
    match expr {
      // 숫자 리터럴 (예: 10)
      crate::Expr::Number(n) => {
        self.emit(format!("PUSH {}", n));    // 리터럴 값을 스택에 푸시
      }

      // 변수 참조 (예: x)
      crate::Expr::Variable(name) => {
        self.emit(format!("LOAD {}", name)); // 변수의 값을 메모리에서 불러와 스택에 푸시
      }

      // 이항 연산 (예: x + 1;, a < b)
      crate::Expr::BinaryOp(left, op, right) => {
        self.gen_expr(left);  // 왼쪽 피연산자 평가
        self.gen_expr(right); // 오른쪽 피연산자 평가

        // 연산자 종류에 따라 명령어 배치
        match op.as_str() {
          "+" => self.emit("ADD"), // 두 값 더하기
          "-" => self.emit("SUB"), // 두 값 빼기
          "==" => self.emit("EQ"), // 같음 비교 -> 0 또는 1 푸시
          "<" => self.emit("LT"),  // 작음 비교
          ">" => self.emit("GT"),  // 큼 비교
          other => {
            // 아직 구현 안된 연산자일 경우 주석으로 표시해 디버깅에 도움
            self.emit(format!("// UNKNOWN_OP {}", other));
          } 
        }
      }
    }
  }
}