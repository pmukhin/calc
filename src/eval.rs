use crate::ast::Expr;

pub struct Eval {}

pub enum Error {}

impl Eval {
    pub fn new() -> Self {
        Self {}
    }

    pub fn exec(&self, ast: &Expr) -> Result<i32, Error> {
        match ast {
            Expr::Number(n) => Ok(*n),
            Expr::Mul(l, r) => Ok(self.exec(l)? * self.exec(r)?),
            Expr::Add(l, r) => Ok(self.exec(l)? + self.exec(r)?),
            Expr::Sub(l, r) => Ok(self.exec(l)? - self.exec(r)?),
            Expr::Div(l, r) => Ok(self.exec(l)? / self.exec(r)?),
        }
    }
}
