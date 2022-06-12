#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
}

pub type ParseResult<T> = Result<Result<T, ParseError>, ParseError>;
