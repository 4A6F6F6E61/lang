//use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
mod lexer;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Token {
    Function(token::Function),
    LoopFunction(token::Function),
    Loop(token::Loop),
    Const(token::Let),
    Global(token::Let),
    Var(token::Let),
    Number(i64),
    String(String),
    OpCode(String),
    Port(String),
    Comment(String),
    If(token::If),
    Else(token::Else),
    ElseIf(token::If),
    End(token::Br),
    OpenSqBr(token::Br),
    CloseSqBr(token::Br),
    OpenRoBr(token::Br),
    CloseRoBr(token::Br),
    CImport(String),
    Line(Vec<Token>),
    Operator(token::Operator),
    Generic(String),
    Stack,
    Accumulator,
    Comma,
    Yield,
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
pub mod token {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Function {
        pub name: String,
        pub arguments: Vec<Arg>,
        pub return_type: String,
        pub lines: Vec<super::Line>,
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
        pub level: i32,
    }
    impl If {
        pub fn new(condition: String, id: i32, level: i32) -> Self {
            Self {
                condition,
                level,
                id: id + level,
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Else {
        pub level: i32,
        pub id: i32,
    }
    impl Else {
        pub fn new(id: i32, level: i32) -> Self {
            Self {
                level,
                id: id + level,
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Br {
        pub level: i32,
        pub id: i32,
    }
    impl Br {
        pub fn new(id: i32, level: i32) -> Self {
            Self {
                level,
                id: id + level,
            }
        }
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
