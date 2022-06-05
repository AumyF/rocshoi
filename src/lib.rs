#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    IntegerLiteral(String),
    HexIntegerLiteral(String),
    Operator(String),
    LowerIdentifier(String),
    UpperIdentifier(String),
    Keyword(String),
    EOF,
}

fn tokenize(input: impl Into<String>) -> Vec<Token> {
    let input: String = input.into();

    let mut chars = input.chars().peekable();

    let mut tokens = Vec::new();

    while let Some(char) = chars.next() {
        if char.is_lowercase() && char.is_alphabetic() {
            let mut store = String::new();
            let mut char = char;
            loop {
                store.push(char);

                // TODO rewrite using chars.peek().is_some_and()
                match chars.peek() {
                    Some(&next) if next.is_alphanumeric() => {
                        char = next;
                        chars.next();
                    }
                    _ => {
                        let token = Token::LowerIdentifier(store);
                        tokens.push(token);
                        break;
                    }
                }
            }
        } else if char.is_digit(10) && char != '0' {
            let mut store = String::new();
            let mut char = char;

            loop {
                store.push(char);

                match chars.peek() {
                    Some(&next) if next.is_digit(10) => {
                        char = next;
                        chars.next();
                    }
                    _ => {
                        let token = Token::IntegerLiteral(store);
                        tokens.push(token);
                        break;
                    }
                }
            }
        } else if char == '0' {
            let second = chars.peek();
            match second {
                Some(&radix) if radix == 'x' => {
                    let mut store = String::new();
                    store.push(char);
                    store.push(radix);
                    chars.next();

                    loop {
                        match chars.peek() {
                            Some(&next) if next.is_digit(16) => {
                                store.push(next);
                                chars.next();
                            }
                            _ => {
                                let token = Token::HexIntegerLiteral(store);
                                tokens.push(token);
                                break;
                            }
                        }
                    }
                }
                Some(&second) => panic!("invalid radix: {}", second),
                None => panic!("unexpected EOF"),
            }
        }
    }

    tokens.push(Token::EOF);

    tokens
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
}
