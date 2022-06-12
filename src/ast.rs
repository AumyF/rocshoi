#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    IntegerLiteral(usize),
    Add(Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Import {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Script {
    pub expressions: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDeclaration {
    pub name: String,
    pub body: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceFile {
    Script(Script),
    Module(Module),
}
