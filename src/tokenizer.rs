use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    IntegerLiteral(String),
    HexIntegerLiteral(String),
    StringLiteral(String),
    Operator(String),
    LowerIdentifier(String),
    UpperIdentifier(String),
    Keyword(String),
    ParenLeft,
    ParenRight,
    BraceLeft,
    BraceRight,
    EOF,
}

struct Tokenizer<'a> {
    tokens: Vec<Token>,
    chars: Peekable<Chars<'a>>,
    store: String,
}

impl Tokenizer<'_> {
    fn new(input: &str) -> Tokenizer {
        let tokens = Vec::new();
        let chars = input.chars().peekable();
        let store = String::new();
        Tokenizer {
            chars,
            tokens,
            store,
        }
    }
    fn take_while(
        &mut self,
        predicate: impl Fn(char) -> bool,
        make_token: impl FnOnce(String) -> Token,
    ) {
        loop {
            match self.chars.peek() {
                Some(&next) if predicate(next) => {
                    self.store.push(next);
                    self.chars.next();
                }
                _ => {
                    let token = make_token(self.store.clone());
                    self.tokens.push(token);
                    self.store.clear();
                    break;
                }
            }
        }
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokenizer = Tokenizer::new(input);

    while let Some(char) = tokenizer.chars.next() {
        if char.is_lowercase() && char.is_alphabetic() {
            tokenizer.store.push(char);

            tokenizer.take_while(char::is_alphanumeric, Token::LowerIdentifier)
        } else if char.is_digit(10) && char != '0' {
            tokenizer.store.push(char);

            tokenizer.take_while(|c| c.is_digit(10), Token::IntegerLiteral)
        } else if char == '0' {
            let second = tokenizer.chars.peek();
            match second {
                Some(&radix) if radix == 'x' => {
                    tokenizer.store.push(char);
                    tokenizer.store.push(radix);
                    tokenizer.chars.next();

                    tokenizer.take_while(|c| c.is_digit(16), Token::HexIntegerLiteral);
                }
                Some(&second) => panic!("invalid radix: {}", second),
                None => panic!("unexpected EOF"),
            }
        } else if char == '"' {
            loop {
                match tokenizer.chars.peek() {
                    Some(&next) if next != '"' => {
                        tokenizer.store.push(next);
                        tokenizer.chars.next();
                    }
                    Some(_) => {
                        let token = Token::StringLiteral(tokenizer.store.clone());
                        tokenizer.tokens.push(token);
                        tokenizer.store.clear();
                        tokenizer.chars.next(); // throw '"' away

                        break;
                    }
                    None => panic!("Unexpected EOF"),
                }
            }
            tokenizer.store.clear();
        } else if char == '(' {
            tokenizer.tokens.push(Token::ParenLeft);
        } else if char == ')' {
            tokenizer.tokens.push(Token::ParenRight);
        } else if char == '{' {
            tokenizer.tokens.push(Token::BraceLeft);
        } else if char == '}' {
            tokenizer.tokens.push(Token::BraceRight);
        }
    }

    tokenizer.tokens.push(Token::EOF);

    tokenizer.tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    #[test]
    fn def_main() {
        let tokens = tokenize(r#"def main"#);
        assert_eq!(
            tokens,
            vec![
                LowerIdentifier("def".to_string()),
                LowerIdentifier("main".to_string()),
                Token::EOF,
            ]
        )
    }

    #[test]
    fn integer() {
        let tokens = tokenize(r#"123 321"#);
        assert_eq!(
            tokens,
            vec![
                IntegerLiteral("123".to_string()),
                IntegerLiteral("321".to_string()),
                EOF,
            ]
        )
    }

    #[test]
    fn hex_integer() {
        let tokens = tokenize(r#"0x123 0xe38182"#);
        assert_eq!(
            tokens,
            vec![
                HexIntegerLiteral("0x123".to_string()),
                HexIntegerLiteral("0xe38182".to_string()),
                EOF,
            ]
        )
    }

    #[test]
    fn string() {
        let tokens = tokenize(r#""Hello, world""#);
        assert_eq!(
            tokens,
            vec![StringLiteral("Hello, world".to_string()), EOF,]
        )
    }
}
