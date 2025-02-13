use crate::ast::Expr::{Add, Div, Mul, Number, Sub};
use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i32),
    Mul(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InsufficientInput,
    UnexpectedToken(Token),
}

impl Expr {
    fn from_expr_and_tok(left: Expr, tok: &[Token]) -> Result<Expr, Error> {
        if tok.is_empty() {
            return Ok(left);
        }
        if tok[0].number().is_some() {
            return Err(Error::UnexpectedToken(tok[0].clone()));
        }
        match tok[0] {
            Token::Number(_) => {
                panic!("we can't be here")
            }
            Token::Plus => Ok(Add(Box::new(left), Box::new(Expr::from(&tok[1..])?))),
            Token::Minus => Ok(Sub(Box::new(left), Box::new(Expr::from(&tok[1..])?))),
            Token::Asterisk => {
                let right = tok[1].number().ok_or(Error::UnexpectedToken(tok[1]))?;
                let expr = Mul(Box::new(left), Box::new(Number(right)));
                Self::from_expr_and_tok(expr, &tok[2..])
            }
            Token::Slash => {
                let right = tok[1].number().ok_or(Error::UnexpectedToken(tok[1]))?;
                let expr = Div(Box::new(left), Box::new(Number(right)));
                Self::from_expr_and_tok(expr, &tok[2..])
            }
        }
    }

    pub fn from(tok: &[Token]) -> Result<Expr, Error> {
        if tok.len() == 0 {
            return Err(Error::InsufficientInput);
        }
        if tok.len() == 1
            && let Some(n) = tok[0].number()
        {
            return Ok(Number(n));
        }
        if tok.len() == 2 {
            return Err(Error::InsufficientInput);
        }
        let left = tok[0].number().ok_or(Error::UnexpectedToken(tok[0]))?;
        let right = tok[2].number().ok_or(Error::UnexpectedToken(tok[2]))?;

        match tok[1] {
            Token::Number(_) => Err(Error::UnexpectedToken(tok[1])),
            Token::Plus => Ok(Add(
                Box::new(Number(left)),
                Box::new(Self::from(&tok[2..])?),
            )),
            Token::Minus => Ok(Sub(
                Box::new(Number(left)),
                Box::new(Self::from(&tok[2..])?),
            )),
            Token::Asterisk => {
                let expr = Mul(Box::new(Number(left)), Box::new(Number(right)));
                Self::from_expr_and_tok(expr, &tok[3..])
            }
            Token::Slash => {
                let expr = Div(Box::new(Number(left)), Box::new(Number(right)));
                Self::from_expr_and_tok(expr, &tok[3..])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Expr;
    use crate::ast::Expr::Number;
    use crate::token::Token;

    #[test]
    fn test_parse_number() {
        let tokens = vec![Token::Number(42)];
        assert_eq!(Expr::from(tokens.as_slice()).unwrap(), Expr::Number(42));
    }

    #[test]
    fn test_parse_addition() {
        let tokens = vec![Token::Number(42), Token::Plus, Token::Number(11)];
        assert_eq!(
            Expr::from(tokens.as_slice()).unwrap(),
            Expr::Add(Box::new(Expr::Number(42)), Box::new(Expr::Number(11)),)
        );
    }

    #[test]
    fn test_parse_recursive_addition() {
        let tokens = vec![
            Token::Number(42),
            Token::Plus,
            Token::Number(11),
            Token::Plus,
            Token::Number(27),
        ];
        assert_eq!(
            Expr::from(tokens.as_slice()).unwrap(),
            Expr::Add(
                Box::new(Expr::Number(42)),
                Box::new(Expr::Add(
                    Box::new(Expr::Number(11)),
                    Box::new(Expr::Number(27))
                )),
            )
        );
    }

    #[test]
    fn test_parse_recursive_subtraction() {
        let tokens = vec![
            Token::Number(42),
            Token::Minus,
            Token::Number(11),
            Token::Plus,
            Token::Number(27),
        ];
        assert_eq!(
            Expr::from(tokens.as_slice()).unwrap(),
            Expr::Sub(
                Box::new(Expr::Number(42)),
                Box::new(Expr::Add(
                    Box::new(Expr::Number(11)),
                    Box::new(Expr::Number(27))
                )),
            )
        );
    }

    #[test]
    fn test_parse_simple_multiplication() {
        let tokens = vec![Token::Number(42), Token::Asterisk, Token::Number(11)];
        assert_eq!(
            Expr::from(tokens.as_slice()).unwrap(),
            Expr::Mul(Box::new(Expr::Number(42)), Box::new(Expr::Number(11)),)
        );
    }

    #[test]
    fn test_parse_simple_division() {
        let tokens = vec![Token::Number(42), Token::Slash, Token::Number(11)];
        assert_eq!(
            Expr::from(tokens.as_slice()).unwrap(),
            Expr::Div(Box::new(Expr::Number(42)), Box::new(Expr::Number(11)),)
        );
    }

    #[test]
    fn test_parse_recursive_multiplication() {
        let tokens = vec![
            Token::Number(4),
            Token::Asterisk,
            Token::Number(7),
            Token::Plus,
            Token::Number(2),
        ];
        assert_eq!(
            Expr::from(tokens.as_slice()).unwrap(),
            Expr::Add(
                Box::new(Expr::Mul(
                    Box::new(Expr::Number(4)),
                    Box::new(Expr::Number(7))
                )),
                Box::new(Expr::Number(2)),
            )
        );
    }

    #[test]
    fn test_parse_recursive_division() {
        let tokens = vec![
            Token::Number(4),
            Token::Slash,
            Token::Number(7),
            Token::Plus,
            Token::Number(2),
        ];
        assert_eq!(
            Expr::from(tokens.as_slice()).unwrap(),
            Expr::Add(
                Box::new(Expr::Div(
                    Box::new(Expr::Number(4)),
                    Box::new(Expr::Number(7))
                )),
                Box::new(Expr::Number(2)),
            )
        );
    }

    #[test]
    fn test_parse_recursive_multiplication_and_division() {
        let tokens = vec![
            Token::Number(4),
            Token::Slash,
            Token::Number(7),
            Token::Asterisk,
            Token::Number(2),
            Token::Plus,
            Token::Number(1),
        ];
        assert_eq!(
            Expr::from(tokens.as_slice()).unwrap(),
            Expr::Add(
                Box::new(Expr::Mul(
                    Box::new(Expr::Div(Box::new(Number(4)), Box::new(Number(7)))),
                    Box::new(Expr::Number(2))
                )),
                Box::new(Expr::Number(1)),
            )
        );
    }
}
