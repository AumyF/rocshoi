use super::token::Token;
use std::iter::Peekable;
use std::str::Chars;

struct Tokenizer<'a> {
    tokens: Vec<Token>,
    chars: Peekable<Chars<'a>>,
}

impl Tokenizer<'_> {
    fn new(input: &str) -> Tokenizer {
        let tokens = Vec::new();
        let chars = input.chars().peekable();
        Tokenizer {
            chars,
            tokens,
        }
    }

    fn take_while(&mut self, store: &mut String, predicate: impl Fn(char) -> bool) {
        loop {
            match self.chars.peek() {
                Some(&next) if predicate(next) => {
                    store.push(next);
                    self.chars.next();
                }
                _ => break,
            }
        }
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokenizer = Tokenizer::new(input);

    while let Some(char) = tokenizer.chars.next() {
        if char.is_lowercase() && char.is_alphabetic() {
            let mut store = String::new();
            store.push(char);

            tokenizer.take_while(&mut store, char::is_alphanumeric);
            let token = Token::LowerIdentifier(store);
            tokenizer.tokens.push(token);
        } else if char.is_digit(10) && char != '0' {
            let mut store = String::new();
            store.push(char);

            tokenizer.take_while(&mut store, |c| c.is_digit(10));
            let token = Token::IntegerLiteral(store);
            tokenizer.tokens.push(token);
        } else if char == '0' {
            let second = tokenizer.chars.peek();
            match second {
                Some(&radix) if radix == 'x' => {
                    let mut store = String::new();
                    store.push(char);
                    store.push(radix);
                    tokenizer.chars.next();

                    tokenizer.take_while(&mut store, |c| c.is_digit(16));
                    let token = Token::HexIntegerLiteral(store);
                    tokenizer.tokens.push(token);
                }
                Some(&second) => panic!("invalid radix: {}", second),
                None => panic!("unexpected EOF"),
            }
        } else if char == '"' {
            let mut store = String::new();
            loop {
                match tokenizer.chars.peek() {
                    Some(&next) if next != '"' => {
                        store.push(next);
                        tokenizer.chars.next();
                    }
                    Some(_) => {
                        let token = Token::StringLiteral(store);
                        tokenizer.tokens.push(token);

                        tokenizer.chars.next(); // throw '"' away

                        break;
                    }
                    None => panic!("Unexpected EOF"),
                }
            }
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
