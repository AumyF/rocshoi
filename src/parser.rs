use super::ast::*;
use super::token::Token;
use std::iter::Peekable;
mod parse_result;
use parse_result::*;

pub struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    fn read_next(&mut self, label: &str) -> Result<&Token, ParseError> {
        self.tokens.peek().ok_or(ParseError {
            message: format!("Expected {label}, but got EOF"),
        })
    }

    fn expect_and_trash(&mut self, wanted: Token, label: &str) -> Result<(), ParseError> {
        let got = self.tokens.next().ok_or(ParseError {
            message: format!("Expected {label}, but got EOF"),
        })?;

        match got {
            got if got == wanted => Ok(()),
            got => Err(ParseError {
                message: format!("Expceted {label}, but got {:?}", got),
            }),
        }
    }

    fn get_left_paren(&mut self) -> Result<(), ParseError> {
        self.expect_and_trash(Token::ParenLeft, "'('")
    }

    fn get_right_paren(&mut self) -> Result<(), ParseError> {
        self.expect_and_trash(Token::ParenRight, "')'")
    }

    fn get_lower_identifier(&mut self) -> Result<String, ParseError> {
        match self.tokens.peek().ok_or(ParseError {
            message: format!("Expected identifier, but got EOF"),
        })? {
            Token::LowerIdentifier(identifier) => Ok(identifier.clone().to_string()),
            token => Err(ParseError {
                message: format!("Expected identifier, but got {:?}", token),
            }),
        }
    }

    pub fn function_declaration(
        &mut self,
    ) -> ParseResult<FunctionDeclaration> {
        match self.tokens.peek().ok_or(ParseError {
            message: "Expected def got EOF".to_string(),
        })? {
            Token::Def => {
                self.tokens.next(); // throw 'def' away

                let name = self.get_lower_identifier()?;
                self.tokens.next();

                self.get_left_paren()?;

                // TODO function types, multiple function params
                let param = self.get_lower_identifier()?;
                self.tokens.next();

                self.get_right_paren()?;

                self.expect_and_trash(Token::BraceLeft, "'{'")?;
                // TODO parse function body
                self.expect_and_trash(Token::BraceRight, "'}'")?;

                Ok(Ok(FunctionDeclaration {
                    name,
                    body: Vec::new(),
                }))
            }
            _ => Err(ParseError{message:"Expected def".to_string()}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_declr() {
        let p = crate::tokenizer::Tokenizer::new("def main(foo) {}");
        let mut parser = Parser {
            tokens: p.peekable(),
        };
        assert_eq!(
            parser.function_declaration().unwrap().unwrap(),
            FunctionDeclaration {
                name: "main".to_string(),
                body: Vec::new(),
            }
        );
    }
}
