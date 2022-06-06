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
