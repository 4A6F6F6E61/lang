//use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
mod lexer;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Token {
    /**
     * Standard Tokens
     */
    Function(token::Function),
    LoopFunction(token::Function),
    GeneratorFunction(token::Function),
    Loop(token::Loop),
    Const(token::Let),
    Global(token::Let),
    Var(token::Let),
    If(token::If),
    Else(token::Else),
    ElseIf(token::If),
    For(token::For),
    End(token::Br),
    OpenSqBr(token::Br),
    CloseSqBr(token::Br),
    OpenRoBr(token::Br),
    CloseRoBr(token::Br),
    Yield(token::Expression),
    Assign(token::Assign),
    Return(token::Expression),
    /**
     * Standard Types
     */
    Number(i64),
    String(String),
    OpCode(String),
    Port(String),
    Comment(String),
    CImport(String),
    Generic(String),
    Line(Vec<Token>),
    /**
     * Expression Tokens
     */
    Expression(token::Expression),
    Operator(token::expression::Operator),
    ExpVal(String),
    /**
     * Other
     */
    Comma,
    Empty,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Line {
    pub tokens: Vec<Token>,
    pub as_string: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FunctionType {
    Function,
    Loop,
    Generator,
}

// -----------------------------------------------------------------------
// Token structs
// -----------------------------------------------------------------------
pub mod token {
    use serde::{Deserialize, Serialize};

    use super::Token;
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
    pub struct For {
        pub id: i32,
        pub level: i32,
        pub var_name: String,
        pub iter_exp: Expression,
    }
    impl For {
        pub fn new(var_name: String, iter_exp: Expression, id: i32, level: i32) -> Self {
            Self {
                var_name,
                iter_exp,
                level,
                id: id + level,
            }
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Let {
        pub name: String,
        pub exp: Expression,
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct If {
        pub condition: Expression,
        pub id: i32,
        pub level: i32,
    }
    impl If {
        pub fn new(condition: Expression, id: i32, level: i32) -> Self {
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
    pub struct Assign {
        pub var: String,
        pub exp: Expression,
    }

    pub mod expression {
        use serde::{Deserialize, Serialize};
        #[derive(Clone, Debug, Deserialize, Serialize)]
        pub enum Operator {
            BitShiftRight,
            BitShiftLeft,
            And,
            Or,
            BitAnd,
            BitOr,
            Plus,
            Minus,
            Mul,
            Div,
            Equals,
            Pipe,
        }
    }

    pub type Expression = Vec<Token>;
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
