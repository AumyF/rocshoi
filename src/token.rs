#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    // Literal
    IntegerLiteral(String),
    HexIntegerLiteral(String),
    StringLiteral(String),

    // Identifier
    LowerIdentifier(String),
    UpperIdentifier(String),
    
    // Keywords
    Let,
    Def,

    // Punctuators
    Plus,
    Minus,
    Colon,
    Equal,
    
    // Delimiters
    ParenLeft,
    ParenRight,
    BraceLeft,
    BraceRight,
  
    // EOF
    EOF,
}
