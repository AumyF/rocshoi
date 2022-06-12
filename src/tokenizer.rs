use super::token::Token;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
    sentEOF: bool,
}

impl Tokenizer<'_> {
    pub fn new(input: &str) -> Tokenizer {
        let chars = input.chars().peekable();
        Tokenizer {
            chars,
            sentEOF: false,
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

    fn skip_space(&mut self) {
        loop {
            let char = self.chars.peek();

            match char {
                Some(ws) if ws.is_whitespace() => self.chars.next(),
                _ => break,
            };
        }
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.skip_space();
        if let Some(char) = self.chars.next() {
            if char.is_lowercase() && char.is_alphabetic() {
                let mut store = String::new();
                store.push(char);

                self.take_while(&mut store, char::is_alphanumeric);

                let token = match &store[..] {
                    "def" => Token::Def,
                    _ => Token::LowerIdentifier(store),
                };
                Some(token)
            } else if char.is_digit(10) && char != '0' {
                let mut store = String::new();
                store.push(char);

                self.take_while(&mut store, |c| c.is_digit(10));
                let token = Token::IntegerLiteral(store);
                Some(token)
            } else if char == '0' {
                let second = self.chars.peek();
                match second {
                    Some(&radix) if radix == 'x' => {
                        let mut store = String::new();
                        store.push(char);
                        store.push(radix);
                        self.chars.next();

                        self.take_while(&mut store, |c| c.is_digit(16));
                        let token = Token::HexIntegerLiteral(store);
                        Some(token)
                    }
                    Some(&second) => panic!("invalid radix: {}", second),
                    None => panic!("unexpected EOF"),
                }
            } else if char == '"' {
                let mut store = String::new();
                let token = loop {
                    match self.chars.peek() {
                        Some(&next) if next != '"' => {
                            store.push(next);
                            self.chars.next();
                        }
                        Some(_) => {
                            let token = Token::StringLiteral(store);

                            self.chars.next(); // throw '"' away

                            break token;
                        }
                        None => panic!("Unexpected EOF"),
                    }
                };
                Some(token)
            } else if char == '(' {
                Some(Token::ParenLeft)
            } else if char == ')' {
                Some(Token::ParenRight)
            } else if char == '{' {
                Some(Token::BraceLeft)
            } else if char == '}' {
                Some(Token::BraceRight)
            } else if char == '+' {
                Some(Token::Plus)
            } else {
                panic!("Unexpected character {}", char)
            }
        } else {
            if self.sentEOF {
                None
            } else {
                self.sentEOF = true;
                Some(Token::EOF)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    #[test]
    fn def_main() {
        let tokens: Vec<Token> = Tokenizer::new(r#"def main(arg) {}"#).collect();
        assert_eq!(
            tokens,
            vec![
                Def,
                LowerIdentifier("main".to_string()),
                ParenLeft,
                LowerIdentifier("arg".to_string()),
                ParenRight,
                BraceLeft,
                BraceRight,
                EOF,
            ]
        )
    }

    #[test]
    fn integer() {
        let tokens: Vec<Token> = Tokenizer::new(r#"123 321"#).collect();
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
        let tokens: Vec<Token> = Tokenizer::new(r#"0x123 0xe38182"#).collect();
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
        let tokens: Vec<Token> = Tokenizer::new(r#""Hello, world""#).collect();
        assert_eq!(
            tokens,
            vec![StringLiteral("Hello, world".to_string()), EOF,]
        )
    }
}
