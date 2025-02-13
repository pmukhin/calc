use crate::ast::Expr;

pub struct Eval {}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_mul() {
        let expr = Expr::Mul(
            Box::new(Expr::Add(Box::new(Expr::Number(5)), Box::new(Expr::Number(6)))),
            Box::new(Expr::Sub(Box::new(Expr::Number(10)), Box::new(Expr::Number(3)))),
        );
        let result = Eval::new().exec(&expr);
        assert_eq!(result.unwrap(), 77);
    }

    #[test]
    fn test_eval_div() {
        let expr = Expr::Div(
            Box::new(Expr::Add(Box::new(Expr::Number(5)), Box::new(Expr::Number(6)))),
            Box::new(Expr::Sub(Box::new(Expr::Number(10)), Box::new(Expr::Number(3)))),
        );
        let result = Eval::new().exec(&expr);
        assert_eq!(result.unwrap(), 1);
    }
}
