#![feature(let_chains)]

use crate::ast::Expr;
use crate::eval::Eval;
use crate::token::Token;

mod ast;
mod eval;
mod token;

pub struct Error(String);

impl From<token::Error> for Error {
    fn from(e: token::Error) -> Error {
        match e {
            token::Error::UnexpectedChar(pos, ch) => {
                Error(format!("unexpected character '{}' as pos {}", ch, pos))
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error(format!("IO error: {}", e.to_string()))
    }
}

impl From<ast::Error> for Error {
    fn from(e: ast::Error) -> Error {
        match e {
            ast::Error::InsufficientInput => {
                Error("insufficient input".to_string())
            }
            ast::Error::UnexpectedToken(tok) => {
                Error(format!("unexpected token {:?}", tok))
            }
        }
    }
}

impl From<eval::Error> for Error {
    fn from(_value: eval::Error) -> Self {
        // defined for future use
        panic!("not defined yet.")
    }
}

fn repl(eval: &Eval) -> Result<(), Error> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    let tokens = Token::tokenize(line.as_str())?;
    let ast = Expr::from(&tokens[..])?;
    let result = eval.exec(&ast)?;

    println!("{}", result);
    Ok(())
}

fn main() {
    let calc = Eval::new();

    loop {
        match repl(&calc) {
            Ok(()) => continue,
            Err(msg) => println!("error occurred: {}", msg.0),
        }
    }
}
