#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    IntegerLiteral(String),
    Operator(String),
    LowerIdentifier(String),
    UpperIdentifier(String),
    Keyword(String),
    EOF,
}

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const NON_ZERO_DIGITS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

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
        }
    }

    tokens.push(Token::EOF);

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn def_main() {
        use Token::*;

        let tokens = tokenize(r#"def main"#);
        assert_eq!(
            tokens,
            vec![
                Token::LowerIdentifier("def".to_string()),
                Token::LowerIdentifier("main".to_string()),
                Token::EOF,
            ]
        )
    }
}
