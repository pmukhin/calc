#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Asterisk,
    Slash,
}

#[derive(Debug)]
pub enum Error {
    UnexpectedChar(usize, char),
}

impl Token {
    pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut chars = input.chars().peekable();
        let mut pos: usize = 0;

        while let Some(c) = chars.next() {
            if c.is_whitespace() {
                pos += 1;
                continue;
            }
            if c == '-' {
                pos += 1;
                tokens.push(Token::Minus);
                continue;
            }
            if c == '+' {
                pos += 1;
                tokens.push(Token::Plus);
                continue;
            }
            if c == '*' {
                pos += 1;
                tokens.push(Token::Asterisk);
                continue;
            }
            if c == '/' {
                pos += 1;
                tokens.push(Token::Slash);
                continue;
            }

            if c.is_ascii_digit() {
                let mut s = String::new();
                s.push(c);

                while let Some(p) = chars.peek()
                    && p.is_ascii_digit()
                {
                    s.push(*p);
                    chars.next();
                }

                pos += s.len();
                tokens.push(Token::Number(
                    s.parse::<i32>()
                        .map_err(|_| Error::UnexpectedChar(pos, c))?,
                ));
                continue;
            }
        }

        Ok(tokens)
    }

    pub fn number(&self) -> Option<i32> {
        if let Token::Number(n) = self {
            Some(*n)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Token;

    #[test]
    fn tokenize_number() {
        assert_eq!(Token::tokenize("  1234   ").unwrap(), vec![Token::Number(
            1234
        )]);
    }

    #[test]
    fn tokenize_minus() {
        assert_eq!(Token::tokenize("  -   ").unwrap(), vec![Token::Minus]);
    }

    #[test]
    fn tokenize_plus() {
        assert_eq!(Token::tokenize(" +   ").unwrap(), vec![Token::Plus]);
    }

    #[test]
    fn tokenize_asterisk() {
        assert_eq!(Token::tokenize(" *   ").unwrap(), vec![Token::Asterisk]);
    }

    #[test]
    fn tokenize_slash() {
        assert_eq!(Token::tokenize("     /").unwrap(), vec![Token::Slash]);
    }

    #[test]
    fn tokenize_expr() {
        assert_eq!(Token::tokenize("   283 * 11 + 5 - 4 / 2 ").unwrap(), vec![
            Token::Number(283),
            Token::Asterisk,
            Token::Number(11),
            Token::Plus,
            Token::Number(5),
            Token::Minus,
            Token::Number(4),
            Token::Slash,
            Token::Number(2),
        ]);
    }
}
