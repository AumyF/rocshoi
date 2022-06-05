use std::iter::Peekable;

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

fn take_while(
    mut store: String,
    tokens: &mut Vec<Token>,
    chars: &mut Peekable<impl Iterator<Item = char>>,
    predicate: impl Fn(char) -> bool,
    make_token: impl FnOnce(String) -> Token,
) {
    loop {
        match chars.peek() {
            Some(&next) if predicate(next) => {
                store.push(next);
                chars.next();
            }
            _ => {
                let token = make_token(store);
                tokens.push(token);
                break;
            }
        }
    }
}

fn tokenize(input: impl Into<String>) -> Vec<Token> {
    let input: String = input.into();

    let mut chars = input.chars().peekable();

    let mut tokens = Vec::new();

    while let Some(char) = chars.next() {
        if char.is_lowercase() && char.is_alphabetic() {
            let mut store = String::new();
            store.push(char);

            take_while(
                store,
                &mut tokens,
                &mut chars,
                char::is_alphanumeric,
                Token::LowerIdentifier,
            );
        } else if char.is_digit(10) && char != '0' {
            let mut store = String::new();
            store.push(char);

            take_while(
                store,
                &mut tokens,
                &mut chars,
                |c| c.is_digit(10),
                Token::IntegerLiteral,
            );
        } else if char == '0' {
            let second = chars.peek();
            match second {
                Some(&radix) if radix == 'x' => {
                    let mut store = String::new();
                    store.push(char);
                    store.push(radix);
                    chars.next();

                    take_while(
                        store,
                        &mut tokens,
                        &mut chars,
                        |c| c.is_digit(16),
                        Token::HexIntegerLiteral,
                    );
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
