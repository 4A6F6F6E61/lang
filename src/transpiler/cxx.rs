#![allow(dead_code)]

use crate::{
    lexer::{
        token::{expression::Operator, Expression, Function},
        Lexer, Token,
    },
    log, printx,
    transpiler::Cxx,
    PrintT,
};
use std::{fs::read_to_string, path::Path};

impl Cxx {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn run<P>(&mut self, file: P)
    where
        P: AsRef<Path>,
    {
        let mut lexer = Lexer::new();
        if let Ok(code) = read_to_string(file) {
            lexer.parse(code);
            //dbg!(&lexer.ast);
            self.transpile(lexer.ast);
        } else {
            log!(CXX, "Unable to read file");
        }
    }

    fn transpile(&mut self, ast: Vec<Token>) {
        let mut ast_iter = ast.iter().peekable();
        while ast_iter.peek().is_some() {
            let nt = ast_iter.next().unwrap();
            match nt {
                Token::Const(x) => {
                    let (name, exp) = (x.name.clone(), x.exp.clone());
                    self.buffer
                        .push_str(&format!("const auto {name} = {};\n", CXX_expression(exp)));
                }
                Token::Global(x) => {
                    let (name, exp) = (x.name.clone(), x.exp.clone());
                    self.buffer.push_str(&format!("auto {name} = {};\n", CXX_expression(exp)));
                }
                Token::Function(x) => {
                    self.function(x, false);
                }
                Token::LoopFunction(x) => {
                    self.function(x, true);
                }
                Token::CImport(ci) => {
                    self.buffer.push_str(&format!("{ci}\n"));
                }
                Token::Comment(_) => {}
                _ => {
                    log!(Error, f("unexpected Token"));
                    dbg!(&nt);
                }
            }
        }
    }
    fn function(&mut self, x: &Function, loop_: bool) {
        let (name, arguments, return_type) =
            (x.name.clone(), x.arguments.clone(), x.return_type.clone());

        let mut args_v = vec![];

        for arg in arguments {
            args_v.push(format!("{} {}", arg.type_, arg.name));
        }

        let args = args_v.join(", ");

        self.buffer
            .push_str(&format!("{return_type} {name} ({args})\n{{\n"));
        if loop_ {
            self.buffer.push_str("do {\n");
        }

        for line in x.lines.clone() {
            let mut token_iter = line.tokens.iter().peekable();
            let mut semic = false;
            while token_iter.peek().is_some() {
                let token = token_iter.next().unwrap();
                match token {
                    Token::If(_if) => {
                        let condition = CXX_expression(_if.condition.clone());
                        self.buffer.push_str(&format!("if ({condition})\n{{\n"));
                    }
                    Token::ElseIf(_if) => {
                        let condition = CXX_expression(_if.condition.clone());
                        self.buffer
                            .push_str(&format!("else if ({condition})\n{{\n"));
                    }
                    Token::Assign(_assign) => {
                        let name = _assign.var.clone();
                        let exp = CXX_expression(_assign.exp.clone());
                        self.buffer
                            .push_str(&format!("{name} = {exp};\n"));
                    }
                    Token::Return(_return) => {
                        let exp = CXX_expression(_return.clone());
                        self.buffer
                            .push_str(&format!("return {exp};\n"));
                    }
                    Token::Var(_var) => {
                        let (name, exp) = (&_var.name, _var.exp.clone());
                        self.buffer.push_str(&format!("auto {name} = {};\n", CXX_expression(exp)));
                    }
                    Token::End(_) => {
                        self.buffer.push_str("}\n");
                    }
                    Token::Else(_) => {
                        self.buffer.push_str("else{\n");
                    }
                    Token::Empty => {
                        self.buffer.push_str("\n");
                    }
                    Token::Generic(s) => {
                        self.buffer.push_str(&format!("{s} "));
                        semic = true;
                    }
                    Token::OpenRoBr(_) => {
                        self.buffer.push_str(&format!("("));
                        semic = true;
                    }
                    Token::CloseRoBr(_) => {
                        self.buffer.push_str(&format!(")"));
                        semic = true;
                    }
                    _ => {
                        log!(Error, "Unexpected Token");
                        dbg!(&token);
                    }
                }
            }
            if semic {
                self.buffer.push_str(";\n");
            }
        }

        if loop_ {
            self.buffer.push_str("} while (1);\n}\n");
        } else {
            self.buffer.push_str("}\n");
        }
    }
}

#[allow(non_snake_case)]
fn CXX_expression(exp: Expression) -> String {
    let mut string = String::new();
    for x in exp {
        match x {
            Token::Operator(o) => match o {
                Operator::Plus => {
                    string.push_str(&format!("+"));
                }
                Operator::Minus => {
                    string.push_str(&format!("-"));
                }
                Operator::Mul => {
                    string.push_str(&format!("*"));
                }
                Operator::Div => {
                    string.push_str(&format!("/"));
                }
                Operator::BitShiftLeft => {
                    string.push_str(&format!("<<"));
                }
                Operator::BitShiftRight => {
                    string.push_str(&format!(">>"));
                }
                Operator::Equals => {
                    string.push_str(&format!("=="));
                }
                Operator::And => {
                    string.push_str(&format!("&&"));
                }
                Operator::Or => {
                    string.push_str(&format!("||"));
                }
                Operator::BitAnd => {
                    string.push_str(&format!("&"));
                }
                Operator::BitOr => {
                    string.push_str(&format!("|"));
                }
                Operator::Pipe => {
                    log!(CXX, "Operator::Pipe : not yet implemented");
                }
            },
            Token::ExpVal(s) => {
                string.push_str(&s);
            }
            _ => {
                log!(Error, "Unexpected token in Expression");
            }
        }
    }
    string
}