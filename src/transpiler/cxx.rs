#![allow(dead_code)]

use crate::{
    lexer::{Function, Lexer, Operator, Token},
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
            dbg!(&lexer.ast);
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
                    let (name, value) = (x.name.clone(), x.value.clone());
                    self.buffer
                        .push_str(&format!("const auto {name} = {value};\n"));
                }
                Token::Global(x) => {
                    let (name, value) = (x.name.clone(), x.value.clone());
                    self.buffer.push_str(&format!("auto {name} = {value};\n"));
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
                        let condition = &_if.condition;
                        self.buffer.push_str(&format!("if ({condition})\n{{\n"));
                    }
                    Token::Var(_var) => {
                        let (name, value) = (&_var.name, &_var.value);
                        self.buffer.push_str(&format!("auto {name} = {value};\n"));
                    }
                    Token::End(_) => {
                        self.buffer.push_str("}\n");
                    }
                    Token::Empty => {
                        self.buffer.push_str("\n");
                    }
                    Token::Generic(s) => {
                        self.buffer.push_str(&format!("{s} "));
                        semic = true;
                    }
                    Token::Operator(o) => {
                        match o {
                            Operator::Plus => {
                                self.buffer.push_str(&format!("+ "));
                            }
                            Operator::Minus => {
                                self.buffer.push_str(&format!("- "));
                            }
                            Operator::Mul => {
                                self.buffer.push_str(&format!("* "));
                            }
                            Operator::Div => {
                                self.buffer.push_str(&format!("/ "));
                            }
                            Operator::BitShiftLeft => {
                                self.buffer.push_str(&format!("<< "));
                            }
                            Operator::BitShiftRight => {
                                self.buffer.push_str(&format!(">> "));
                            }
                            Operator::Equals => {
                                self.buffer.push_str(&format!("= "));
                            }
                        }
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
