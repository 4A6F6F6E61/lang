//use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
mod lexer;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Token {
    Function(Function),
    LoopFunction(Function),
    Loop(Loop),
    Const(Let),
    Global(Let),
    Var(Let),
    Number(i64),
    String(String),
    OpCode(String),
    Port(String),
    Comment(String),
    If(If),
    End(i32),
    OpenSqBr(i32),
    CloseSqBr(i32),
    OpenRoBr(i32),
    CloseRoBr(i32),
    CImport(String),
    Line(Vec<Token>),
    Operator(Operator),
    Stack,
    Accumulator,
    Comma,
    Generic(String),
    Empty,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Line {
    pub tokens: Vec<Token>,
    pub as_string: String,
}

// -----------------------------------------------------------------------
// Token structs
// -----------------------------------------------------------------------
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<Arg>,
    pub return_type: String,
    pub lines: Vec<Line>,
    pub tmp_lines: Vec<Vec<String>>,
    pub start_ln: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Loop {
    pub id: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Let {
    pub name: String,
    pub value: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct If {
    pub condition: String,
    pub id: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Arg {
    pub name: String,
    pub type_: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Operator {
    BitShiftRight,
    BitShiftLeft,
    Plus,
    Minus,
    Mul,
    Div,
    Equals,
}

// -----------------------------------------------------------------------
// Lexer structs
// -----------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Brackets {
    pub round: i32,
    pub square: i32,
    pub braces: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Lexer {
    pub tmp_ast: Vec<Token>,
    pub ast: Vec<Token>,
    strings: Vec<Vec<String>>,
    //progress_bar: ProgressBar,
    brackets: Brackets,
}
